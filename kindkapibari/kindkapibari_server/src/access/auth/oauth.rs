use chrono::{DateTime, Duration, Utc};
use kindkapibari_core::secret::{decode_gotten_secret, DecodedSecret};
use kindkapibari_core::{dbarray::DBArray, dbvec::DBVec, secret::generate_signed_key};
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
    requested_scopes: Vec<Scope>,
) -> AResult<OAuthWithRefresh> {
    if !requested_scopes.is_empty() {
        return Err(Report::msg("Scopes must not be empty."));
    }
    if !requested_scopes.contains(&Scope::OfflineRead) {
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
    requested_scopes: Vec<Scope>,
) -> AResult<String> {
    if !requested_scopes.is_empty() {
        return Err(Report::msg("Scopes must not be empty."));
    }
    if requested_scopes.contains(&Scope::OfflineRead) {
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
        pub scopes: DBVec<Scope>,
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
