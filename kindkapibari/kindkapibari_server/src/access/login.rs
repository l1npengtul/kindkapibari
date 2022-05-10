use crate::schema::users::{login_tokens, oauth_authorizations};
use crate::scopes::Scopes;
use crate::{AResult, AppData};
use chrono::{Duration, TimeZone, Utc};
use color_eyre::Report;
use kindkapibari_core::dbarray::DBArray;
use kindkapibari_core::dbvec::DBVec;
use kindkapibari_core::secret_gen::generate_signed_key;
use kindkapibari_core::snowflake::SnowflakeIdGenerator;
use once_cell::sync::Lazy;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::sync::Arc;

const AUTH_REDIS_KEY_START_OAUTH_ACCESS: [u8; 2] = *b"oa";
const AUTH_REDIS_KEY_START_OAUTH_REFRESH: [u8; 2] = *b"or";
const AUTH_REDIS_KEY_START_SESSION: [u8; 2] = *b"se";
const OAUTH_ACCESS_PREFIX_NO_DASH: &'static str = "OA";
const OAUTH_REFRESH_PREFIX_NO_DASH: &'static str = "OR";
const LOGIN_TOKEN_PREFIX_NO_DASH: &'static str = "LT";

static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

pub fn generate_login_token(state: Arc<AppData>, user_id: u64) -> AResult<String> {
    let config = state.config.read().await;
    let key = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let token = format!(
        "{}.{}.{}-{}",
        base64::encode(key.nonce),
        LOGIN_TOKEN_PREFIX_NO_DASH,
        base64::encode(&key.salt),
        base64::encode(&key.signed)
    );

    let now = Utc::now();

    let login_token_active = login_tokens::ActiveModel {
        owner: ActiveValue::Set(user_id),
        expire: ActiveValue::Set(now.add(Duration::days(69))),
        created: ActiveValue::Set(now),
        session_hashed: ActiveValue::Set(key.signed),
        salt: ActiveValue::Set(DBArray::from(key.salt)),
        ..Default::default()
    };
    login_token_active.insert(&state.database).await?;

    Ok(token)
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct OAuthWithRefresh {
    pub access: String,
    pub refresh: String,
}

pub fn generate_oauth_with_refresh(
    state: Arc<AppData>,
    user_id: u64,
    application_id: u64,
    requested_scopes: Vec<Scopes>,
) -> AResult<OAuthWithRefresh> {
    if !requested_scopes.is_empty() {
        return Err(Report::msg("Scopes must not be empty."));
    }
    if !requested_scopes.contains(&Scopes::OfflineRead) {
        return Err(Report::msg(
            "To have refresh token you must add the `OfflineRead` scope.",
        ));
    }

    let config = state.config.read().await;
    let access = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;
    let refresh = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let access_token = format!(
        "{}.{}.{}-{}",
        base64::encode(access.nonce),
        OAUTH_ACCESS_PREFIX_NO_DASH,
        base64::encode(&access.salt),
        base64::encode(&access.signed)
    );
    let refresh_token = format!(
        "{}.{}.{}-{}",
        base64::encode(refresh.nonce),
        OAUTH_REFRESH_PREFIX_NO_DASH,
        base64::encode(&refresh.salt),
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
        scopes: ActiveValue::Set(DBVec::from(requested_scopes)),
        ..Default::default()
    };

    oauth_token_active.insert(&state.database).await?;

    Ok(OAuthWithRefresh {
        access: access_token,
        refresh: refresh_token,
    })
}

pub fn generate_oauth_no_refresh(
    state: Arc<AppData>,
    user_id: u64,
    application_id: u64,
    requested_scopes: Vec<Scopes>,
) -> AResult<String> {
    if !requested_scopes.is_empty() {
        return Err(Report::msg("Scopes must not be empty."));
    }
    if requested_scopes.contains(&Scopes::OfflineRead) {
        return Err(Report::msg(
            "To not have refresh token you must not request the `OfflineRead` scope.",
        ));
    }

    let config = state.config.read().await;
    let access = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let access_token = format!(
        "{}.{}.{}-{}",
        base64::encode(access.nonce),
        OAUTH_ACCESS_PREFIX_NO_DASH,
        base64::encode(&access.salt),
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
        scopes: ActiveValue::Set(DBVec::from(requested_scopes)),
        ..Default::default()
    };

    oauth_token_active.insert(&state.database).await?;

    Ok(access_token)
}
