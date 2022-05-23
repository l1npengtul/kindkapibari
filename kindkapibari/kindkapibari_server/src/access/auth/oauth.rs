use crate::{
    access::TOKEN_SEPERATOR,
    roles::Roles,
    schema::applications,
    user,
    users::{oauth_authorizations, AuthorizedUser},
    AResult, AppData, KKBScope, Report, SResult, ServerError,
};
use chrono::{DateTime, Duration, Utc};
use kindkapibari_core::{
    dbarray::DBArray,
    dbvec::DBVec,
    secret::{decode_gotten_secret, generate_signed_key, DecodedSecret},
};
use oxide_auth::endpoint::{Authorizer, Issuer, PreGrant, Registrar};
use oxide_auth::primitives::grant::Grant;
use oxide_auth::primitives::issuer::{IssuedToken, RefreshedToken};
use oxide_auth::primitives::prelude::ClientUrl;
use oxide_auth::primitives::registrar::{BoundClient, RegistrarError};
use redis::AsyncCommands;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::sync::Arc;
use tracing::instrument;

pub const AUTH_REDIS_KEY_START_OAUTH_ACCESS: [u8; 2] = *b"oa";
pub const AUTH_REDIS_KEY_START_OAUTH_REFRESH: [u8; 2] = *b"or";
pub const OAUTH_ACCESS_PREFIX_NO_DASH: &'static str = "OA";
pub const OAUTH_REFRESH_PREFIX_NO_DASH: &'static str = "OR";

pub struct KKBOAuthState {}

pub struct OAuthAuthorizer {}

impl Authorizer for OAuthAuthorizer {
    fn authorize(&mut self, _: Grant) -> Result<String, ()> {
        todo!()
    }

    fn extract(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        todo!()
    }
}

pub struct OAuthRegistrar {}

impl Registrar for OAuthRegistrar {
    fn bound_redirect<'a>(&self, bound: ClientUrl<'a>) -> Result<BoundClient<'a>, RegistrarError> {
        todo!()
    }

    fn negotiate(
        &self,
        client: BoundClient,
        scope: Option<oxide_auth::endpoint::Scope>,
    ) -> Result<PreGrant, RegistrarError> {
        todo!()
    }

    fn check(&self, client_id: &str, passphrase: Option<&[u8]>) -> Result<(), RegistrarError> {
        todo!()
    }
}

pub struct OAuthIssuer {}

impl Issuer for OAuthIssuer {
    fn issue(&mut self, grant: Grant) -> Result<IssuedToken, ()> {
        todo!()
    }

    fn refresh(&mut self, _refresh: &str, _grant: Grant) -> Result<RefreshedToken, ()> {
        todo!()
    }

    fn recover_token<'a>(&'a self, _: &'a str) -> Result<Option<Grant>, ()> {
        todo!()
    }

    fn recover_refresh<'a>(&'a self, _: &'a str) -> Result<Option<Grant>, ()> {
        todo!()
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
            .await?
            .ok_or(ServerError::NotFound("user", "database"))?;

        let authorized_user = AuthorizedUser {
            scopes: requested_scopes,
            user,
        };

        if let Ok(secret) =
            decode_gotten_secret(&token, TOKEN_SEPERATOR, config.signing_key.as_bytes())
        {
            state.caches.oauth_token.insert(secret, authorized_user);
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
            .await?
            .ok_or(ServerError::NotFound("user", "database"))?;

        let authorized_user = AuthorizedUser {
            scopes: requested_scopes,
            user,
        };

        if let Ok(secret) =
            decode_gotten_secret(&token, TOKEN_SEPERATOR, config.signing_key.as_bytes())
        {
            state.caches.oauth_token.insert(secret, authorized_user);
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
pub async fn application_by_id(state: Arc<AppData>, id: u64) -> SResult<applications::Model> {
    if let Ok(application) = state.caches.applications.get(&id) {
        return Ok(application);
    }

    let application_query = applications::Entity::find_by_id(id)
        .one(&state.database)
        .await?
        .ok_or(ServerError::NotFound("No application", "Not Found"))?;
    // commit to cache
    state
        .caches
        .applications
        .insert(id, application_query.clone()); // rip alloc
    Ok(application_query)
}
