use crate::schema::user;
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use once_cell::sync::Lazy;
use poem::Request;
use poem_openapi::auth::ApiKey;
use sha2::{Digest, Sha512};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

const AUTH_REDIS_KEY_START: [u8; 16] = *b"coconutpak:auth:";
const AUTH_SESSION_BYTE_START: &'static str = "SESSION-";
const AUTH_APIKEY_BYTE_START: &'static str = "API-KEY-";

// i love r/196! (turn the bytes to KiB)
static AUTO_RESEEDING_APIKEY_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_SESSION_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_UUID_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

const fn uuid_to_byte_array(uuid: Uuid) -> [u8; 16] {
    let u128 = uuid.as_u128();
    u128.to_ne_bytes()
}
// A0 A1 A2 B0
pub async fn generate_apikey(user_uuid: Uuid) -> (String, Vec<u8>) {
    let uuid_as_bytes = uuid_to_byte_array(user_uuid);
    let mut apikey_base = AUTO_RESEEDING_APIKEY_RNG
        .lock()
        .await
        .generate_bytes::<48>()
        .to_vec();
    apikey_base.insert_str(0, &uuid_as_bytes);
    let mut hasher = Sha512::new();
    hasher.update(apikey_base);
    let hashed_key = &(hasher.finalize()[..]);
    let mut encoded = base64::encode(hashed_key);
    encoded.insert_str(0, AUTH_APIKEY_BYTE_START);
    let mut bytes = Vec::from(hashed_key);
    bytes.insert_str(0, AUTH_APIKEY_BYTE_START);
    (encoded, bytes)
}

pub async fn generate_session(user_uuid: Uuid) -> (String, Vec<u8>) {
    let uuid_as_bytes = uuid_to_byte_array(user_uuid);
    let mut session_base = AUTO_RESEEDING_APIKEY_RNG
        .lock()
        .await
        .generate_bytes::<48>()
        .to_vec();
    session_base.insert_str(0, &uuid_as_bytes);
    let mut hasher = Sha512::new();
    hasher.update(session_base);
    let hashed_key = &(hasher.finalize()[..]);
    let mut encoded = base64::encode(hashed_key);
    encoded.insert_str(0, AUTH_SESSION_BYTE_START);
    let mut bytes = Vec::from(hashed_key);
    bytes.insert_str(0, AUTH_SESSION_BYTE_START);
    (encoded, bytes)
}

pub async fn generate_uuid() -> Uuid {
    let uuid_base = AUTO_RESEEDING_SESSION_RNG
        .lock()
        .await
        .generate_bytes::<16>();
    Uuid::from_bytes(uuid_base)
}

pub async fn verify_apikey(request: &Request, api_key: ApiKey) -> Option<user::Model> {}

pub async fn verify_session(request: &Request, api_key: ApiKey) -> Option<user::Model> {}
