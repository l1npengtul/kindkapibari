use crate::reseedingrng::AutoReseedingRng;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use chacha20poly1305::{
    aead::{consts::U24, generic_array::GenericArray, Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use chrono::Utc;
use eyre::Report;
use once_cell::sync::Lazy;
use staticvec::StaticVec;
use std::sync::Arc;
use tokio::sync::Mutex;

static AUTO_RESEEDING_TOKEN_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_NONCE_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_SALT_SHAKER_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    // i love shaking salts all over my hash~~browns~~
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

// Issue Flow
// Generate Token -> GeneratedToken -> SentSecret(signed) -> [{}.{{}}] to Client
//                         |---------> StoredSecret(nonce + salt + raw) -> Database
// Redeem/Verify Flow
// Receive [{}.{{}.{}}] -> SentSecret -> StoredSecret::verify_secret(StoredSecret, SentSecret
#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct GeneratedToken {
    sent: SentSecret,
    store: StoredSecret,
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

    let mut hash: StaticVec<u8, 64> = StaticVec::new();
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
    let nonce_salt = AUTO_RESEEDING_SALT_SHAKER_RNG
        .lock()
        .await
        .generate_bytes::<16>();
    let mut nonce: StaticVec<u8, 24> = StaticVec::new();
    argon2_nonce.hash_password_into(&pre_nonce, &nonce_salt, &mut nonce)?;

    // sign the key
    let key = Key::from_slice(signing_key);
    let aead = XChaCha20Poly1305::new(key);
    let nonce_generic = XNonce::from_slice(&nonce);
    let signed = aead.encrypt(nonce_generic, &base)?;

    Ok(Self {
        sent: SentSecret { signed },
        store: StoredSecret { hash, salt, nonce },
    })
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentSecret {
    pub signed: Vec<u8>,
}

impl SentSecret {
    pub fn from_str_token(token: impl AsRef<str>) -> Option<Self> {
        let signed = base64::decode(token.as_ref()).ok()?;

        Some(Self { signed })
    }

    pub fn to_str_token(&self) -> String {
        base64::encode(&self.signed)
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredSecret {
    pub hash: StaticVec<u8, 64>,
    pub salt: [u8; 32],
    pub nonce: StaticVec<u8, 24>,
}

impl StoredSecret {
    pub fn nonce(&self) -> &GenericArray<u8, U24> {
        XNonce::from_slice(&self.nonce)
    }

    pub fn verify(&self, sent: &SentSecret, signing_key: impl AsRef<[u8]>) -> bool {
        let decrypted = match aead(signing_key).decrypt(self.nonce(), &sent.signed) {
            Ok(data) => data,
            Err(_) => return false,
        };

        let raw = match base64::decode(String::from_utf8(decrypted).ok()?.as_ref()).ok() {
            Some(r) => r,
            None => return false,
        };

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
        let mut decoded_hash = Vec::with_capacity(64);
        if let Err(()) = argon2.hash_password_into(&raw, &self.salt, &mut decoded_hash) {
            return false;
        }

        decoded_hash == self.hash
    }
}

#[cfg(feature = "server")]
crate::impl_redis!(StoredSecret, SentSecret);
#[cfg(feature = "server")]
crate::impl_sea_orm!(StoredSecret, SentSecret);

fn aead(signing_key: impl AsRef<[u8]>) -> XChaCha20Poly1305 {
    let key = Key::from_slice(signing_key.as_ref());
    XChaCha20Poly1305::new(key)
}
