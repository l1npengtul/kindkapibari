use crate::schema::user;
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use once_cell::sync::Lazy;
use poem::Request;
use poem_openapi::auth::ApiKey;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

const AUTH_REDIS_KEY_START: [u8; 16] = *b"coconutpak:auth:";

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
// A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15 B0
pub async fn generate_apikey(user_uuid: Uuid) -> (String, Vec<u8>) {
    let uuid_as_bytes = uuid_to_byte_array(user_uuid);
    let apikey_base = AUTO_RESEEDING_APIKEY_RNG
        .lock()
        .await
        .generate_bytes::<48>()
        .chunks(16);
}

pub async fn generate_session(user_uuid: Uuid) -> (String, Vec<u8>) {}

pub async fn generate_uuid() -> Uuid {}

pub async fn verify_apikey(request: &Request, api_key: ApiKey) -> Option<user::Model> {}

pub async fn verify_session(request: &Request, api_key: ApiKey) -> Option<user::Model> {}
