use crate::schema::api_key::{Column, Entity, Model};
use crate::schema::{api_key, session, user};
use crate::{AppData, SResult};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use bson::spec::BinarySubtype::Column;
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use once_cell::sync::Lazy;
use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::types::Type;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisResult};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Related};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::fmt::{write, Display, Formatter};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

const AUTH_REDIS_KEY_START_APIKEY: [u8; 22] = *b"coconutpak:auth:apikey";
const AUTH_REDIS_KEY_START_SESSION: [u8; 23] = *b"coconutpak:auth:session";
const AUTH_SESSION_BYTE_START: &'static str = "COCONUTPAK_SESSION_TOKEN.";
const AUTH_APIKEY_BYTE_START: &'static str = "COCONUTPAK_APIKEY_TOKEN.";

// i love r/196! (turn the bytes to KiB)
static AUTO_RESEEDING_APIKEY_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_SESSION_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_UUID_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Generated {
    pub raw_bytes: Vec<u8>,
    pub hash_salt_bytes: Vec<u8>,
}

const fn uuid_to_byte_array(uuid: Uuid) -> [u8; 16] {
    let u128 = uuid.as_u128();
    u128.to_le_bytes()
}

pub async fn generate_key(is_api_key: bool) -> SResult<Generated> {
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

    Ok(Generated {
        raw_bytes: base,
        hash_salt_bytes: hash,
    })
}

pub async fn generate_uuid() -> Uuid {
    let uuid_base = AUTO_RESEEDING_SESSION_RNG
        .lock()
        .await
        .generate_bytes::<16>();
    Uuid::from_bytes(uuid_base)
}

pub async fn verify_apikey(database: Arc<AppData>, api_key: String) -> Option<user::Model> {
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

    let rehashed_key = base64::decode(api_key)
        .map(|mut bytes| {
            let mut hash = Vec::with_capacity(64);
            argon2
                .hash_password_into(&bytes, &salt, &mut hash)
                .map(|| hash)
                .ok()
        })
        .ok()
        .flatten()?;

    // check if redis has our key
    if let Ok(user) = database
        .redis
        .get::<[&[u8]; 2], Option<user::Model>>([&AUTH_REDIS_KEY_START_APIKEY, &rehashed_key])
        .await
    {
        return user;
    }

    return match api_key::Entity::find()
        .filter(Column::KeyHashedSha512.eq(&rehashed_key))
        .one(&database.database)
        .await
        .ok()
        .flatten()
    {
        Some(api_key_model) => {
            let user_model = user::Entity::find_by_id(api_key_model.owner)
                .one(&database.database)
                .await
                .ok()
                .flatten();
            let _result = database
                .redis
                .set([&AUTH_REDIS_KEY_START_APIKEY, &rehashed_key], &user_model)
                .await;
            database
                .redis
                .expire([&AUTH_REDIS_KEY_START_APIKEY, &rehashed_key], 3600)
                .await;
            user_model
        }
        None => None,
    };
}

pub async fn verify_session(database: Arc<AppData>, session: String) -> Option<user::Model> {
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

    let rehashed_key = base64::decode(session)
        .map(|bytes| {
            let mut hash = Vec::with_capacity(64);
            argon2
                .hash_password_into(&bytes, &salt, &mut hash)
                .map(|| hash)
                .ok()
        })
        .ok()
        .flatten()?;

    // check if redis has our key
    if let Ok(user) = database
        .redis
        .get::<[&[u8]; 2], Option<user::Model>>([&AUTH_REDIS_KEY_START_SESSION, &rehashed_key])
        .await
    {
        return user;
    }

    return match session::Entity::find()
        .filter(session::Column::SessionHashedSha512.eq(&rehashed_key))
        .one(&database.database)
        .await
        .ok()
        .flatten()
    {
        Some(session_key_model) => {
            let user_model = user::Entity::find_by_id(session_key_model.owner)
                .one(&database.database)
                .await
                .ok()
                .flatten();
            let _result = database
                .redis
                .set([&AUTH_REDIS_KEY_START_SESSION, &rehashed_key], &user_model)
                .await;
            database
                .redis
                .expire([&AUTH_REDIS_KEY_START_SESSION, &rehashed_key], 3600)
                .await;
            user_model
        }
        None => None,
    };
}
