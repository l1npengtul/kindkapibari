use crate::appdata_traits::AppData;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use kindkapibari_core::scopes::make_kkbscope_scope;
use kindkapibari_core::{
    reseedingrng::AutoReseedingRng,
    secret::{GeneratedToken, SentSecret},
    snowflake::SnowflakeIdGenerator,
};
use oxide_auth::{
    endpoint::{PreGrant, Scope},
    frontends::dev::Url,
    primitives::{
        grant::Grant,
        issuer::{IssuedToken, RefreshedToken, TokenType},
        prelude::ClientUrl,
        registrar::{BoundClient, ExactUrl, RegisteredUrl, RegistrarError},
    },
};
use oxide_auth_async::primitives::{Authorizer, Issuer, Registrar};
use redis::{aio::ConnectionManager, AsyncCommands};
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, ops::Add, str::FromStr, sync::Arc};
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

        if let Ok(_) = grant_from_id(self.state.clone(), hashed.as_str()) {
            return Err(());
        }
        match insert_grant_with_id(self.state.clone(), hashed.as_str(), &grant) {
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

        Ok(PreGrant {
            client_id: client.client_id.into_owned(),
            redirect_uri: client.redirect_uri.into_owned(),
            scope: make_kkbscope_scope(app.scopes.as_slice()),
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
                .map_err(RegistrarError::Unspecified)?,
        )
        .await
        .map_err(RegistrarError::Unspecified)?;

        if application.confidential {
            match passphrase.map(|x| String::from_utf8_lossy(x)) {
                Some(secret) => {
                    let sentsecret =
                        SentSecret::from_str_token(secret).ok_or(RegistrarError::Unspecified)?;
                    // check against stored
                    if let Some(stored) = application.signed_secret {
                        if stored.verify(
                            &sentsecret,
                            self.state
                                .config
                                .read()
                                .await
                                .signing_keys
                                .oauth_key
                                .as_bytes(),
                        ) {
                            return Ok(());
                        }
                    }
                    Err(RegistrarError::Unspecified)
                }
                None => Err(RegistrarError::Unspecified),
            }
        } else {
            Ok(())
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
        // see if the cache has our token
        let sent = match SentSecret::from_str_token(access) {
            Some(s) => s,
            None => return Ok(None),
        };
        let config = self.state.config.read().await;
        let oauth_auth = match self.state.caches.access_tokens_cache.get(&sent) {
            Some(s) => match s {
                Some(m) => m,
                None => return Ok(None),
            },
            None => {
                // query database
                let user_id = match sent.user_id(config.signing_key_oauth.as_bytes()) {
                    Some(uid) => uid,
                    None => return Ok(None),
                };
                match oauth_authorizations_by_user(self.state.clone(), user_id).await {
                    Ok(auths) => match auths
                        .into_iter()
                        .filter(|x| {
                            x.access_token
                                .verify(&sent, config.signing_key_oauth.as_bytes())
                        })
                        .nth(0)
                    {
                        Some(a) => a,
                        None => return Ok(None),
                    },
                    Err(_) => return Ok(None),
                }
            }
        };

        // query application
        let application = match application_by_id(self.state.clone(), oauth_auth.application).await
        {
            Ok(app) => app,
            Err(_) => return Ok(None),
        };

        Ok(Some(Grant {
            owner_id: format!("{}", oauth_auth.owner),
            client_id: format!("{}", oauth_auth.application),
            scope: make_kkbscope_scope(application.scopes.as_slice()),
            redirect_uri: Url::parse(&application.callback).unwrap(),
            until: oauth_auth.expire,
            extensions: Default::default(),
        }))
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

    let keys = &state.config.read().await.signing_keys;
    let access = GeneratedToken::new(user_id, keys.oauth_key.as_bytes())?;
    let refresh = GeneratedToken::new(user_id, keys.oauth_key.as_bytes())?;
    let now = Utc::now();

    let oauth_token_active = oauth_authorizations::ActiveModel {
        owner: ActiveValue::Set(user_id),
        application: ActiveValue::Set(application_id),
        expire: ActiveValue::Set(now.add(Duration::hours(3))),
        created: ActiveValue::Set(now),
        access_token: ActiveValue::Set(access.store),
        refresh_token: ActiveValue::Set(Some(refresh.store)),
        ..Default::default()
    };

    oauth_token_active.insert(&state.database).await?;

    Ok(OAuthWithRefresh {
        access: access.sent.to_string(),
        refresh: access.sent.to_string(),
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

    let keys = &state.config.read().await.signing_keys;
    let access = GeneratedToken::new(user_id, keys.oauth_key.as_bytes())?;
    let now = Utc::now();

    let oauth_token_active = oauth_authorizations::ActiveModel {
        owner: ActiveValue::Set(user_id),
        application: ActiveValue::Set(application_id),
        expire: ActiveValue::Set(now.add(Duration::hours(3))),
        created: ActiveValue::Set(now),
        access_token: ActiveValue::Set(access.store),
        ..Default::default()
    };

    oauth_token_active.insert(&state.database).await?;
    Ok(access.sent.to_string())
}

// #[instrument]
// pub async fn verify_access_token(state: Arc<AppData>, token: SentSecret) -> SResult<AuthorizedUser> {
//     if let Ok(user) = state.caches.access_tokens_cache.get(&token) {
//         return Ok(user);
//     }
//
//     #[derive(FromQueryResult)]
//     struct AuthorizedUserQuery {
//         pub id: u64,
//         pub username: String,
//         pub handle: String,
//         pub email: Option<String>,
//         pub profile_picture: Option<String>,
//         pub creation_date: DateTime<Utc>,
//         pub roles: DBVec<Roles>,
//         pub scopes: DBVec<KKBScope>,
//     }
//
//     let access = oauth_authorizations::Entity::find()
//         .filter(oauth_authorizations::Column::AccessTokenSalt.eq(&token.salt))
//         .filter(oauth_authorizations::Column::Expire.gt(Utc::now()))
//         .join(JoinType::Join, oauth_authorizations::Relation::User.def())
//         .into_model::<AuthorizedUserQuery>()
//         .one(&state.database)
//         .await?
//         .ok_or(ServerError::Unauthorized)?;
//
//     Ok(AuthorizedUser {
//         scopes: access.scopes.to_vec(),
//         user: user::Model {
//             id: access.id,
//             username: access.username,
//             handle: access.handle,
//             email: access.email,
//             profile_picture: access.profile_picture,
//             creation_date: access.creation_date,
//             roles: access.roles,
//         },
//     })
// }

#[instrument]
pub async fn oauth_authorizations_by_user(
    state: Arc<AppData>,
    id: u64,
) -> SResult<Vec<oauth_authorizations::Model>> {
    let authorizations = oauth_authorizations::Entity::find()
        .filter(oauth_authorizations::Column::Owner.eq(id))
        .all(&state.database)
        .await?;
    Ok(authorizations)
}

// INTELLIJ-RUST STFU!!!!!!
// THE VARIABLE IS BEING USED
// AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
#[allow(unused_variables)]
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
