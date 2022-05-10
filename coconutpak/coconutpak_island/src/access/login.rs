use crate::{
    permissions::Scopes,
    schema::{api_key, session, user},
    AppData, SResult,
};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
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

const AUTH_REDIS_KEY_START_APIKEY: [u8; 6] = *b"cpk:ak";
const AUTH_REDIS_KEY_START_SESSION: [u8; 6] = *b"cpk:se";
const API_KEY_PREFIX_NO_DASH: &'static str = "A";
const TOKEN_PREFIX_NO_DASH: &'static str = "S";

// 196! 196! 196! 196!
static AUTO_RESEEDING_TOKEN_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_NONCE_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
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

pub fn generate_key(state: Arc<AppData>, is_api_key: bool) -> SResult<Generated> {
    let config = state.config.read().await;
    let mut base = AUTO_RESEEDING_TOKEN_RNG
        .lock()
        .await
        .generate_bytes::<64>()
        .to_vec();

    base.append(&mut Utc::now().timestamp_millis().to_ne_bytes().to_vec());
    base.push(config.machine_id);

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

    let mut hash = Vec::with_capacity(64);
    argon2.hash_password_into(&base, &config.salting.as_bytes(), &mut hash)?;

    // sign the key
    let key = Key::from_slice(config.signing_key.as_bytes());
    let aead = XChaCha20Poly1305::new(key);
    let nonce = XNonce::from_slice(&AUTO_RESEEDING_NONCE_RNG.lock().await.generate_bytes::<24>());
    let signed = aead.encrypt(nonce, hash)?;

    let front = if is_api_key {
        API_KEY_PREFIX_NO_DASH
    } else {
        TOKEN_PREFIX_NO_DASH
    };
    let mut key = format!(
        "{front}.{}.{}",
        base64::encode(nonce),
        base64::encode(&signed)
    );

    Ok(Generated {
        key,
        hashed: signed,
    })
}

pub async fn new_apikey(
    state: Arc<AppData>,
    user_id: u64,
    name: String,
) -> SResult<CoconutPakApiKey> {
    let key = generate_key(state.clone(), true)?;

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
    let key = generate_key(state.clone(), false)?;

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
