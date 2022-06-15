use crate::access::application::application_by_id;
use crate::{
    access::{delet_dis, insert_into_cache, TOKEN_SEPERATOR},
    roles::Roles,
    user,
    users::{oauth_authorizations, AuthorizedUser},
    AResult, AppData, KKBScope, Report, SResult, ServerError,
};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use kindkapibari_core::{
    dbarray::DBArray,
    dbvec::DBVec,
    reseedingrng::AutoReseedingRng,
    secret::{decode_gotten_secret, generate_signed_key, DecodedSecret},
    snowflake::SnowflakeIdGenerator,
};
use oxide_auth::primitives::issuer::TokenType;
use oxide_auth::{
    endpoint::PreGrant,
    primitives::{
        grant::{Grant, Scope},
        issuer::{IssuedToken, RefreshedToken},
        prelude::ClientUrl,
        registrar::{BoundClient, ExactUrl, RegisteredUrl, RegistrarError},
    },
};
use oxide_auth_async::primitives::{Authorizer, Issuer, Registrar};
use redis::{aio::ConnectionManager, AsyncCommands};
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::{borrow::Cow, str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use tracing::instrument;

pub const AUTH_REDIS_KEY_START_OAUTH_ACCESS: [u8; 2] = *b"oa";
pub const AUTH_REDIS_KEY_START_OAUTH_REFRESH: [u8; 2] = *b"or";
pub const AUTH_REDIS_KEY_START_OAUTH_AUTHORIZER: [u8; 4] = *b"oaut";
const OAUTH_GRANT_REDIS_KEY: [u8; 14] = *b"kkboauthgrant:";
pub const OAUTH_ACCESS_PREFIX_NO_DASH: &'static str = "OA";
pub const OAUTH_REFRESH_PREFIX_NO_DASH: &'static str = "OR";

pub struct KKBOAuthState {}

struct RedisHolder(pub ConnectionManager);

#[derive(Clone, Debug)]
pub struct OAuthAuthorizer {
    state: Arc<AppData>,
    rng: Arc<Mutex<AutoReseedingRng<65535>>>,
    id: Arc<Mutex<SnowflakeIdGenerator>>,
    machine_id: u8,
}

#[async_trait]
impl Authorizer for OAuthAuthorizer {
    #[instrument]
    async fn authorize(&mut self, grant: Grant) -> Result<String, ()> {
        let id: [u8; 8] = self.id.lock().await.generate_id().to_le_bytes();
        let rng: [u8; 56] = self.rng.lock().await.generate_bytes();
        let hashed = format!(
            "{AUTH_REDIS_KEY_START_OAUTH_AUTHORIZER}:{}",
            base64::encode(blake3::hash([&rng, &id].concat()).as_bytes())
        );
        let kkbgrant: KKBGrant = grant.into();
        // check with redis

        if let Ok(_) = grant_from_id(self.state.clone(), hashed.as_str()) {
            return Err(());
        }
        match insert_grant_with_id(self.state.clone(), hashed.as_str(), &kkbgrant) {
            Ok(_) => Ok(hashed),
            Err(_) => Err(()),
        }
    }

    #[instrument]
    async fn extract(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        if let Ok(kkbgrant) = grant_from_id(self.state.clone(), token) {
            let grant = delete_grant(self.state.clone(), token).await?;
            if kkbgrant == grant {
                Ok(Some(grant.into()))
            }
            Err(())
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct OAuthRegistrar {
    state: Arc<AppData>,
}

// The "clients" here are the OAuth clients not our users!
#[async_trait]
impl Registrar for OAuthRegistrar {
    #[instrument]
    async fn bound_redirect<'a>(
        &self,
        bound: ClientUrl<'a>,
    ) -> Result<BoundClient<'a>, RegistrarError> {
        let client =
            match application_by_id(self.state.clone(), bound.client_id.parse::<u64>()?).await {
                Ok(app) => app,
                Err(_) => return Err(RegistrarError::Unspecified),
            };

        let registered_url = match bound.redirect_uri {
            Some(ref uri) => {
                if &client.callback == uri.as_str() {
                    RegisteredUrl::from(
                        ExactUrl::from_str(&client.callback)
                            .map_err(RegistrarError::PrimitiveError)?,
                    )
                } else {
                    return Err(RegistrarError::PrimitiveError);
                }
            }
            None => RegisteredUrl::from(
                ExactUrl::from_str(&client.callback).map_err(RegistrarError::PrimitiveError)?,
            ),
        };

        Ok(BoundClient {
            client_id: bound.client_id,
            redirect_uri: Cow::Owned(registered_url),
        })
    }

    #[instrument]
    async fn negotiate(
        &self,
        client: BoundClient,
        _: Option<Scope>,
    ) -> Result<PreGrant, RegistrarError> {
        let app =
            match application_by_id(self.state.clone(), client.client_id.parse::<u64>()?).await {
                Ok(app) => app,
                Err(_) => return Err(RegistrarError::Unspecified),
            };

        // TODO: faster way to do this
        let mut scopes_str = String::new();
        for scope in app.scopes.iter() {
            scopes_str += &scope.to_attr_string();
        }
        let scopes = scopes_str.parse::<Scope>()?;

        Ok(PreGrant {
            client_id: client.client_id.into_owned(),
            redirect_uri: client.redirect_uri.into_owned(),
            scope: scopes,
        })
    }

    #[instrument]
    async fn check(
        &self,
        client_id: &str,
        passphrase: Option<&[u8]>,
    ) -> Result<(), RegistrarError> {
        let application = application_by_id(
            self.state.clone(),
            client_id
                .parse::<u64>()
                .map_err(RegistrarError::PrimitiveError)?,
        )
        .await
        .map_err(RegistrarError::PrimitiveError)?;
        match passphrase {
            Some(secret) => {
                let secret_str =
                    String::from_utf8(Vec::from(secret)).map_err(RegistrarError::PrimitiveError)?;
                let decoded = Some(decode_gotten_secret(
                    secret_str,
                    TOKEN_SEPERATOR,
                    self.state.config.read().await.signing_key.as_bytes(),
                )?);

                if application.signed_secret == decoded {
                    Ok(())
                } else {
                    Err(RegistrarError::Unspecified)
                }
            }
            None => Ok(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OAuthIssuer {
    state: Arc<AppData>,
}

impl OAuthIssuer {
    fn set_duration(&self, grant: &mut Grant) {
        grant.until = Utc::now() + self.state.config.read().await.oauth.default_time;
    }
}

#[async_trait]
impl Issuer for OAuthIssuer {
    #[instrument]
    async fn issue(&mut self, grant: Grant) -> Result<IssuedToken, ()> {
        let mut grant = grant;
        self.set_duration(&mut grant);

        let scopes = grant
            .scope
            .iter()
            .map(|scope| KKBScope::from_str(scope))
            .collect::<Result<Vec<KKBScope>, ()>>()?;

        let generated = generate_oauth_no_refresh(
            self.state.clone(),
            grant.owner_id.parse().map_err(|| ())?,
            grant.client_id.parse().map_err(|| ())?,
            scopes,
        )
        .await
        .map_err(|| ())?;

        Ok(IssuedToken {
            token: generated,
            refresh: None,
            until: grant.until,
            token_type: TokenType::Bearer,
        })
    }

    #[instrument]
    async fn refresh(&mut self, _refresh: &str, _grant: Grant) -> Result<RefreshedToken, ()> {
        // see if exist
        // TODO
        Err(())
    }

    #[instrument]
    async fn recover_token<'a>(&'a self, access: &'a str) -> Result<Option<Grant>, ()> {
        // query database
        
    }

    #[instrument]
    async fn recover_refresh<'a>(&'a self, _: &'a str) -> Result<Option<Grant>, ()> {
        // TODO
        Err(())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct OAuthWithRefresh {
    pub access: String,
    pub refresh: String,
}

#[instrument]
pub async fn generate_oauth_with_refresh(
    state: Arc<AppData>,
    user_id: u64,
    application_id: u64,
    requested_scopes: Vec<KKBScope>,
) -> AResult<OAuthWithRefresh> {
    if !requested_scopes.is_empty() {
        return Err(Report::msg("Scopes must not be empty."));
    }
    if !requested_scopes.contains(&KKBScope::OfflineRead) {
        return Err(Report::msg(
            "To have refresh token you must add the `OfflineRead` scope.",
        ));
    }

    let config = state.config.read().await;
    let access = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;
    let refresh = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let access_token = format!(
        "{}.{}.{}{}{}",
        base64::encode(access.nonce),
        OAUTH_ACCESS_PREFIX_NO_DASH,
        base64::encode(&access.salt),
        TOKEN_SEPERATOR,
        base64::encode(&access.signed)
    );
    let refresh_token = format!(
        "{}.{}.{}{}{}",
        base64::encode(refresh.nonce),
        OAUTH_REFRESH_PREFIX_NO_DASH,
        base64::encode(&refresh.salt),
        TOKEN_SEPERATOR,
        base64::encode(&refresh.signed)
    );

    let now = Utc::now();

    let oauth_token_active = oauth_authorizations::ActiveModel {
        owner: ActiveValue::Set(user_id),
        application: ActiveValue::Set(application_id),
        expire: ActiveValue::Set(now.add(Duration::hours(3))),
        created: ActiveValue::Set(now),
        access_token_hashed: ActiveValue::Set(access.signed),
        access_token_salt: ActiveValue::Set(DBArray::from(access.salt)),
        refresh_token_hashed: ActiveValue::Set(Some(refresh.signed)),
        refresh_token_salt: ActiveValue::Set(Some(DBArray::from(refresh.salt))),
        scopes: ActiveValue::Set(DBVec::from(requested_scopes.clone())),
        ..Default::default()
    };

    oauth_token_active.insert(&state.database).await?;

    tokio::task::spawn(async {
        let user = user::Entity::find_by_id(user_id)
            .one(&state.database)
            .await?;

        if user.is_none() {
            if let Ok(secret) =
                decode_gotten_secret(&token, TOKEN_SEPERATOR, config.signing_key.as_bytes())
            {
                state.caches.oauth_token.insert(secret, None);
            }
        } else {
            let user = user.ok_or(ServerError::NotFound("user".into(), "database".into()))?;
            let authorized_user = AuthorizedUser {
                scopes: requested_scopes,
                user,
            };

            if let Ok(secret) =
                decode_gotten_secret(&token, TOKEN_SEPERATOR, config.signing_key.as_bytes())
            {
                state
                    .caches
                    .oauth_token
                    .insert(secret, Some(authorized_user));
            }
        }
    });

    Ok(OAuthWithRefresh {
        access: access_token,
        refresh: refresh_token,
    })
}

#[instrument]
pub async fn generate_oauth_no_refresh(
    state: Arc<AppData>,
    user_id: u64,
    application_id: u64,
    requested_scopes: Vec<KKBScope>,
) -> AResult<String> {
    if !requested_scopes.is_empty() {
        return Err(Report::msg("Scopes must not be empty."));
    }
    if requested_scopes.contains(&KKBScope::OfflineRead) {
        return Err(Report::msg(
            "To not have refresh token you must not request the `OfflineRead` scope.",
        ));
    }

    let config = state.config.read().await;
    let access = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let access_token = format!(
        "{}.{}.{}{}{}",
        base64::encode(access.nonce),
        OAUTH_ACCESS_PREFIX_NO_DASH,
        base64::encode(&access.salt),
        TOKEN_SEPERATOR,
        base64::encode(&access.signed)
    );

    let now = Utc::now();

    let oauth_token_active = oauth_authorizations::ActiveModel {
        owner: ActiveValue::Set(user_id),
        application: ActiveValue::Set(application_id),
        expire: ActiveValue::Set(now.add(Duration::hours(3))),
        created: ActiveValue::Set(now),
        access_token_hashed: ActiveValue::Set(access.signed),
        access_token_salt: ActiveValue::Set(DBArray::from(access.salt)),
        scopes: ActiveValue::Set(DBVec::from(requested_scopes.clone())),
        ..Default::default()
    };

    oauth_token_active.insert(&state.database).await?;

    tokio::task::spawn(async {
        let user = user::Entity::find_by_id(user_id)
            .one(&state.database)
            .await?;

        if user.is_none() {
            if let Ok(secret) =
                decode_gotten_secret(&token, TOKEN_SEPERATOR, config.signing_key.as_bytes())
            {
                state.caches.oauth_token.insert(secret, None);
            }
        } else {
            let user = user.ok_or(ServerError::NotFound("user".into(), "database".into()))?;
            let authorized_user = AuthorizedUser {
                scopes: requested_scopes,
                user,
            };

            if let Ok(secret) =
                decode_gotten_secret(&token, TOKEN_SEPERATOR, config.signing_key.as_bytes())
            {
                state
                    .caches
                    .oauth_token
                    .insert(secret, Some(authorized_user));
            }
        }
    });

    Ok(access_token)
}

#[instrument]
pub fn verify_access_token(state: Arc<AppData>, token: DecodedSecret) -> SResult<AuthorizedUser> {
    if let Ok(user) = state.caches.oauth_token.get(&token) {
        return Ok(user);
    }

    #[derive(FromQueryResult)]
    struct AuthorizedUserQuery {
        pub id: u64,
        pub username: String,
        pub handle: String,
        pub email: Option<String>,
        pub profile_picture: Option<String>,
        pub creation_date: DateTime<Utc>,
        pub roles: DBVec<Roles>,
        pub scopes: DBVec<KKBScope>,
    }

    let access = oauth_authorizations::Entity::find()
        .filter(oauth_authorizations::Column::AccessTokenSalt.eq(&token.salt))
        .filter(oauth_authorizations::Column::Expire.gt(Utc::now()))
        .join(JoinType::Join, oauth_authorizations::Relation::User.def())
        .into_model::<AuthorizedUserQuery>()
        .one(&state.database)
        .await?
        .ok_or(ServerError::Unauthorized)?;

    Ok(AuthorizedUser {
        scopes: access.scopes.to_vec(),
        user: user::Model {
            id: access.id,
            username: access.username,
            handle: access.handle,
            email: access.email,
            profile_picture: access.profile_picture,
            creation_date: access.creation_date,
            roles: access.roles,
        },
    })
}

#[instrument]
pub async fn grant_from_id(state: Arc<AppData>, id: &str) -> SResult<Grant> {
    let new_id = format!("kkb:oauthgrant:{id}");
    let grant = pot::from_slice::<Grant>(&state.redis.get::<String, Vec<u8>>(new_id).await?)?;
    Ok(grant)
}

#[instrument]
pub async fn insert_grant_with_id(state: Arc<AppData>, id: &str, grant: &Grant) -> SResult<()> {
    let grant_as_bytes = pot::to_vec(grant)?;
    insert_into_cache(state, id, grant_as_bytes, None).await
}

#[instrument]
pub async fn delete_grant(state: Arc<AppData>, id: &str) -> SResult<Grant> {
    Ok(pot::from_slice::<Grant>(delet_dis(state, id).await?)?)
}
