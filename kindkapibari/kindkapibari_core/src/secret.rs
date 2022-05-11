use crate::reseedingrng::AutoReseedingRng;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use chacha20poly1305::aead::{Aead, NewAead};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce};
use chrono::Utc;
use eyre::Report;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

static AUTO_RESEEDING_TOKEN_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_NONCE_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_SALT_SHAKER_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    // i love shaking salts all over my hash~~browns~~
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct RawGenerated {
    pub signed: Vec<u8>,
    pub nonce: Vec<u8>,
    pub raw: Vec<u8>,
    pub hash: Vec<u8>,
    pub salt: [u8; 32],
}

pub fn generate_signed_key(machine_id: u8, signing_key: &[u8]) -> Result<RawGenerated, Report> {
    let now_slice: [u8; 8] = Utc::now().timestamp_millis().to_ne_bytes();
    let mut base = Vec::with_capacity(73);
    base.extend_from_slice(&AUTO_RESEEDING_TOKEN_RNG.lock().await.generate_bytes::<64>());
    base.extend_from_slice(&now_slice);
    base.push(machine_id);

    let argon2_key = Argon2::new(
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
    let salt = *blake3::hash(
        [
            &AUTO_RESEEDING_SALT_SHAKER_RNG
                .lock()
                .await
                .generate_bytes::<23>(),
            &now_slice,
            &[machine_id],
        ]
        .concat(),
    )
    .as_bytes();
    argon2_key.hash_password_into(&base, &salt, &mut hash)?;

    let argon2_nonce = Argon2::new(
        Algorithm::Argon2id,
        Version::default(),
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            Some(24),
        )?,
    );

    let mut pre_nonce = Vec::with_capacity(33);
    pre_nonce.extend_from_slice(&AUTO_RESEEDING_NONCE_RNG.lock().await.generate_bytes::<24>());
    pre_nonce.extend_from_slice(Utc::now().timestamp_millis().to_ne_bytes().as_bytes());
    pre_nonce.push(machine_id);
    let salting = AUTO_RESEEDING_SALT_SHAKER_RNG
        .lock()
        .await
        .generate_bytes::<16>();
    let mut nonce = Vec::with_capacity(24);
    argon2_nonce.hash_password_into(&pre_nonce, &salting, &mut nonce);

    // sign the key
    let key = Key::from_slice(signing_key);
    let aead = XChaCha20Poly1305::new(key);
    let nonce_generic = XNonce::from_slice(&nonce);
    let signed = aead.encrypt(nonce_generic, &hash)?;

    Ok(RawGenerated {
        signed,
        salt,
        nonce,
        raw: base,
        hash,
    })
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct DecodedSecret {
    pub secret_type: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}

pub fn decode_gotten_secret(
    gotten: String,
    seperator: &'static str,
    signing_key: &[u8],
) -> Result<DecodedSecret, Report> {
    let splitted = gotten.split(".").collect::<Vec<_>>();
    if splitted.len() != 3 {
        return Err(Report::msg("gotten not long enough"));
    }
    let nonce = base64::decode(splitted[0])?;
    let secret_type = splitted[1];
    let salt_raw_combo = splitted[2].split(seperator).collect::<Vec<_>>();
    if salt_raw_combo.len() != 2 {
        return Err(Report::msg("gotten not long enough"));
    }

    let salt = base64::decode(salt_raw_combo[0])?;
    let key = Key::from_slice(signing_key);
    let aead = XChaCha20Poly1305::new(key);
    let nonce_generic = XNonce::from_slice(&nonce);
    let raw = aead.decrypt(nonce_generic, base64::decode(salt_raw_combo[1])?)?;

    let argon2_key = Argon2::new(
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
    argon2_key.hash_password_into(&raw, &salt, &mut hash);

    Ok(DecodedSecret {
        secret_type: secret_type.to_string(),
        hash,
        salt,
    })
}
