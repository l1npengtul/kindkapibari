use crate::AppData;
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use chrono::{TimeZone, Utc};
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use kindkapibari_core::snowflake::SnowflakeIdGenerator;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

const AUTH_REDIS_KEY_START_OAUTH: [u8; 6] = *b"kkb:oa";
const AUTH_REDIS_KEY_START_SESSION: [u8; 6] = *b"kkb:se";
const OAUTH_PREFIX_NO_DASH: &'static str = "O";
const TOKEN_PREFIX_NO_DASH: &'static str = "S";

static AUTO_RESEEDING_TOKEN_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_NONCE_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Generated {
    pub key: String,
    pub hashed: Vec<u8>,
}

pub fn generate_key(state: Arc<AppData>, is_token: bool) -> SResult<Generated> {
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

    let front = if is_token {
        TOKEN_PREFIX_NO_DASH
    } else {
        OAUTH_PREFIX_NO_DASH
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

pub fn generate_login_token(state: Arc<AppData>, user_id: u64) -> SResult<String> {}
