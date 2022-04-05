use crate::schema::api_key::{Entity, Model};
use crate::schema::{api_key, user};
use crate::{AppData, SResult};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use once_cell::sync::Lazy;
use poem::Request;
use poem_openapi::auth::ApiKey;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisResult};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Related};
use sha2::{Digest, Sha512};
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

pub struct Generated {
    hashed_base64: String,
    hashed_bytes: Vec<u8>,
}

const fn uuid_to_byte_array(uuid: Uuid) -> [u8; 16] {
    let u128 = uuid.as_u128();
    u128.to_le_bytes()
}
// A0 A1 A2 B0
pub async fn generate_apikey(user_uuid: Uuid) -> SResult<Generated> {
    let apikey_base = AUTO_RESEEDING_APIKEY_RNG
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

    let salt = [
        AUTH_APIKEY_BYTE_START.as_bytes(),
        uuid_to_byte_array(user_uuid).as_slice(),
    ]
    .concat();
    let mut hash = Vec::with_capacity(64);
    argon2.hash_password_into(&apikey_base, &salt, &mut hash)?;

    let encoded = base64::encode(&hash);
    Ok(Generated {
        hashed_base64: encoded,
        hashed_bytes: hash,
    })
}

pub async fn generate_session(user_uuid: Uuid) -> SResult<Generated> {
    let apikey_base = AUTO_RESEEDING_APIKEY_RNG
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

    let salt = [
        AUTH_SESSION_BYTE_START.as_bytes(),
        uuid_to_byte_array(user_uuid).as_slice(),
    ]
    .concat();
    let mut hash = Vec::with_capacity(64);
    argon2.hash_password_into(&apikey_base, &salt, &mut hash)?;

    let encoded = base64::encode(&hash);
    Ok(Generated {
        hashed_base64: encoded,
        hashed_bytes: hash,
    })
}

pub async fn generate_uuid() -> Uuid {
    let uuid_base = AUTO_RESEEDING_SESSION_RNG
        .lock()
        .await
        .generate_bytes::<16>();
    Uuid::from_bytes(uuid_base)
}

pub async fn verify_apikey(
    database: Arc<AppData>,
    _request: &Request,
    api_key: String,
) -> Option<(user::Model, api_key::Model)> {
    let mut api_key = api_key;
    // check if its an APIKEY
    if !api_key.starts_with(AUTH_APIKEY_BYTE_START) {
        return None;
    } else {
        api_key = api_key
            .strip_prefix(AUTH_APIKEY_BYTE_START)
            .map(ToString::to_string)
            .unwrap_or(api_key);
    }
    let user = base64::decode(api_key).map(|bytes| -> Option((user::Model, api_key::Model)) {
        // hash and check in with redis
        let mut hasher = Sha512::new();
        hasher.update(&bytes);
        let hashed_key = Vec::from(&(hasher.finalize()[..]));
        let mut redis_key = hashed_key.clone();
        redis_key.insert_str(0, AUTH_REDIS_KEY_START_APIKEY);
        if let Ok(redis_result) = database.redis.get(&redis_key).await {
            let redis_result: Option<user::Model> = redis_result;
            return redis_result;
        }
        let api_key_model = match api_key::Entity::find()
            .filter(api_key::Column::KeyHashedSha512.eq(hashed_key))
            .all(&database.database)
            .await
        {
            Ok(model) => {
                if model.len() > 1 || model.len() == 0 {
                    return None;
                }
                model[0].clone()
            }
            Err(_) => return None,
        };
        let user_model = match user::Entity::find_by_id(api_key_model.owner)
            .one(&database.database)
            .await
            .ok()
            .flatten()
        {
            Some(m) => {
                database.redis.set(redis_key, &m).await;
                m
            }
            None => {}
        };
        Some((api_key_model, user_model))
    });
    None
}

pub async fn verify_session(request: &Request, session: String) -> Option<user::Model> {}
