use crate::{
    permissions::Scopes,
    schema::{api_key, session, user},
    AppData, SResult,
};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use chrono::{TimeZone, Utc};
use kindkapibari_core::{reseedingrng::AutoReseedingRng, snowflake::SnowflakeIdGenerator};
use once_cell::sync::Lazy;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Add, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

const AUTH_REDIS_KEY_START_APIKEY: [u8; 22] = *b"coconutpak:auth:apikey";
const AUTH_REDIS_KEY_START_SESSION: [u8; 23] = *b"coconutpak:auth:session";
const AUTH_SESSION_BYTE_START: &'static str = "COCONUTPAK_SESSION_TOKEN.";
const AUTH_APIKEY_BYTE_START: &'static str = "COCONUTPAK_APIKEY_TOKEN.";

const API_KEY_PREFIX_NO_DASH: &'static str = "CCPKA";
const TOKEN_PREFIX_NO_DASH: &'static str = "CCSTS";

// 196! 196! 196! 196!
static AUTO_RESEEDING_APIKEY_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_SESSION_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

pub type CoconutPakApiKey = String;
pub type CoconutPakSessionToken = String;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Generated {
    pub key: String,
    pub hashed: Vec<u8>,
}

const fn uuid_to_byte_array(uuid: Uuid) -> [u8; 16] {
    let u128 = uuid.as_u128();
    u128.to_le_bytes()
}

pub fn generate_key(is_api_key: bool) -> SResult<Generated> {
    let base = AUTO_RESEEDING_APIKEY_RNG
        .lock()
        .await
        .generate_bytes::<64>()
        .to_vec();
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::default(),
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            Some(64),
        )?,
    );

    let salt = [(if is_api_key {
        AUTH_APIKEY_BYTE_START
    } else {
        AUTH_SESSION_BYTE_START
    })
    .as_bytes()]
    .concat();

    let mut hash = Vec::with_capacity(64);
    argon2.hash_password_into(&apikey_base, &salt, &mut hash)?;
    let front = if is_api_key {
        API_KEY_PREFIX_NO_DASH
    } else {
        TOKEN_PREFIX_NO_DASH
    };
    let mut key = format!("{front}-{}", base64::encode(base));

    Ok(Generated { key, hashed: hash })
}

pub async fn new_apikey(
    state: Arc<AppData>,
    user_id: u64,
    name: String,
) -> SResult<CoconutPakApiKey> {
    let key = generate_key(true)?;

    let api_key_active = api_key::ActiveModel {
        name: ActiveValue::Set(name),
        owner: ActiveValue::Set(user_id),
        key_hashed: ActiveValue::Set(key.hashed),
        created: ActiveValue::Set(Utc::now()),
        ..Default::default()
    };

    api_key_active.insert(&state.database).await?;
    Ok(key.key)
}

pub async fn new_session(state: Arc<AppData>, user_id: u64) -> SResult<CoconutPakSessionToken> {
    let key = generate_key(false)?;

    let session_token_active = session::ActiveModel {
        owner: ActiveValue::Set(user_id),
        expire: ActiveValue::Set(Utc::now().add(chrono::Duration::days(69))), // haha funny sex number now laugh
        created: ActiveValue::Set(Utc::now()),
        session_hashed: ActiveValue::Set(key.hashed),
        ..Default::default()
    };

    session_token_active.insert(&state.database).await?;
    Ok(key.key)
}

pub async fn generate_id(config: Arc<AppData>) -> Option<u64> {
    ID_GENERATOR.generate_id(config.config.read().await.machine_id)
}

pub enum Authorized {
    ApiKey(u64, Vec<Scopes>),
}

pub async fn verify_apikey(
    database: Arc<AppData>,
    api_key: CoconutPakApiKey,
) -> SResult<Option<user::Model>> {
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::default(),
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            Some(64),
        )?,
    );

    if let Some(key_data) = &api_key.strip_prefix(concat!(API_KEY_PREFIX_NO_DASH, "-")) {
        let rehashed_key = base64::decode(key_data)
            .map(|bytes| {
                let mut hash = Vec::with_capacity(64);
                argon2
                    .hash_password_into(&bytes, &salt, &mut hash)
                    .map(|| hash)
                    .ok()
            })
            .ok()
            .flatten()?;

        return Ok(api_key::Entity::find()
            .filter(api_key::Column::KeyHashed.eq(&rehashed_key))
            .filter(api_key::Column::Revoked.eq(false))
            .join(JoinType::RightJoin, api_key::Relation::User.def())
            .into_model::<user::Model>()
            .one(&database.database)
            .await?);
    }

    return Ok(None);
}

pub async fn verify_session(
    database: Arc<AppData>,
    session: CoconutPakSessionToken,
) -> SResult<Option<user::Model>> {
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::default(),
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            Some(64),
        )?,
    );

    if let Some(token_data) = &session.strip_prefix(concat!(TOKEN_PREFIX_NO_DASH, "-")) {
        let rehashed_key = base64::decode(token_data)
            .map(|bytes| {
                let mut hash = Vec::with_capacity(64);
                argon2
                    .hash_password_into(&bytes, &salt, &mut hash)
                    .map(|| hash)
                    .ok()
            })
            .ok()
            .flatten()?;

        return Ok(session::Entity::find()
            .filter(session::Column::SessionHashed.eq(&rehashed_key))
            .filter(session::Column::Expire.gt(Utc::now()))
            .filter(session::Column::Revoked.eq(false))
            .join(JoinType::RightJoin, session::Relation::User.def())
            .into_model::<user::Model>()
            .one(&database.database)
            .await?);
    }
    return Ok(None);
}
