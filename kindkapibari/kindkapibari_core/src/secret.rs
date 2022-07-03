use crate::reseedingrng::AutoReseedingRng;
use argon2::{Algorithm, Argon2, Params, Version};
use base64::DecodeError;
use chacha20poly1305::{
    aead::{consts::U24, generic_array::GenericArray, Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use chrono::Utc;
use once_cell::sync::Lazy;
use staticvec::StaticVec;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tokio::sync::Mutex;

static AUTO_RESEEDING_TOKEN_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_NONCE_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
// i love shaking salts all over my hash~~browns~~
static AUTO_RESEEDING_SALT_SHAKER_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

// Issue Flow
// Generate Token -> GeneratedToken -> SentSecret(signed) -> [{}.{{}}] to Client
//                         |---------> StoredSecret(nonce + salt + raw) -> Database
// Redeem/Verify Flow
// Receive [{}.{{}.{}}] -> SentSecret -> StoredSecret::verify_secret(StoredSecret, SentSecret
#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedToken {
    pub sent: SentSecret,
    pub store: StoredSecret,
}

impl GeneratedToken {
    pub async fn new(
        user_id: u64,
        signing_key: &[u8],
        machine_id: u8,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let now_slice: [u8; 8] = Utc::now().timestamp_millis().to_ne_bytes();
        let mut base = Vec::with_capacity(73);
        base.extend_from_slice(&AUTO_RESEEDING_TOKEN_RNG.lock().await.generate_bytes::<64>());
        base.extend_from_slice(&now_slice);
        base.extend_from_slice(format!(".{user_id}").as_bytes());

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
            &[
                AUTO_RESEEDING_SALT_SHAKER_RNG
                    .lock()
                    .await
                    .generate_bytes::<32>()
                    .as_slice(),
                &now_slice,
                &user_id.to_le_bytes(),
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
        pre_nonce.extend_from_slice(&Utc::now().timestamp_millis().to_ne_bytes());
        pre_nonce.push(machine_id);
        let nonce_salt = AUTO_RESEEDING_SALT_SHAKER_RNG
            .lock()
            .await
            .generate_bytes::<16>();
        let mut nonce: StaticVec<u8, 24> = StaticVec::new();
        argon2_nonce.hash_password_into(&pre_nonce, &nonce_salt, &mut nonce)?;

        // sign the key
        let aead = aead(signing_key);
        let nonce_generic = XNonce::from_slice(&nonce);
        let signed = aead.encrypt(nonce_generic, base.as_slice())?;

        Ok(Self {
            sent: SentSecret {
                iv: nonce_generic.to_vec(),
                signed,
            },
            store: StoredSecret { hash, salt, nonce },
        })
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentSecret {
    pub iv: Vec<u8>,
    pub signed: Vec<u8>,
}

impl SentSecret {
    #[must_use]
    pub fn from_str_token(token: &str) -> Option<Self> {
        let mut split = token
            .split('.')
            .map(base64::decode)
            .collect::<Result<Vec<Vec<u8>>, DecodeError>>()
            .ok()?;
        if split.len() != 2 {
            return None;
        }

        Some(Self {
            iv: split.pop()?,
            signed: split.pop()?,
        })
    }

    pub fn user_id(&self, signing_key: impl AsRef<[u8]>) -> Option<u64> {
        let nonce = XNonce::from_slice(self.iv.as_slice());
        if let Ok(data) = aead(signing_key).decrypt(nonce, self.signed.as_slice()) {
            return String::from_utf8(data)
                .ok()
                .and_then(|x| x.split('.').next().map(ToString::to_string))?
                .parse::<u64>()
                .ok();
        }

        None
    }

    #[must_use]
    pub fn to_str_token(&self) -> String {
        format!(
            "{}.{}",
            base64::encode(&self.iv),
            base64::encode(&self.signed)
        )
    }
}

impl Display for SentSecret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str_token())
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredSecret {
    pub hash: StaticVec<u8, 64>,
    pub salt: [u8; 32],
    pub nonce: StaticVec<u8, 24>,
}

impl StoredSecret {
    #[must_use]
    pub fn nonce(&self) -> &GenericArray<u8, U24> {
        XNonce::from_slice(&self.nonce)
    }

    pub fn verify(&self, sent: &SentSecret, signing_key: impl AsRef<[u8]>) -> bool {
        let decrypted = match aead(signing_key).decrypt(self.nonce(), sent.signed.as_slice()) {
            Ok(data) => data,
            Err(_) => return false,
        };
        let decode_str = match String::from_utf8(decrypted).ok() {
            Some(s) => s,
            None => return false,
        };
        let raw = match base64::decode(decode_str).ok() {
            Some(r) => r,
            None => return false,
        };

        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::default(),
            match Params::new(
                Params::DEFAULT_M_COST,
                Params::DEFAULT_T_COST,
                Params::DEFAULT_P_COST,
                Some(64),
            ) {
                Ok(params) => params,
                Err(_) => return false,
            },
        );
        let mut decoded_hash = Vec::with_capacity(64);
        if argon2
            .hash_password_into(&raw, &self.salt, &mut decoded_hash)
            .is_err()
        {
            return false;
        }

        decoded_hash.as_slice() == self.hash.as_slice()
    }

    #[must_use]
    pub fn to_bin_str(&self) -> String {
        let stored_str = format!(
            "{}.{}.{}",
            base64::encode(&self.nonce),
            base64::encode(&self.salt),
            base64::encode(&self.hash)
        );
        stored_str
    }

    // pub fn from_bin_str(str: &str) -> Result<Self, ()> {
    //     let mut data = str.as_ref().split(".").map(|x| base64::decode(x)).collect::<Result<Vec<Vec<u8>>, DecodeError>>().map_err(|_| ())?;
    //     if data.len() != 3 {
    //         return Err(())
    //     }
    //     let nonce = data.pop();
    //     let salt = data.pop();
    //     let hash = data.pop();
    //     Ok(Self {
    //         hash: ,
    //         salt: [],
    //         nonce: Default::default()
    //     })
    // }
}

#[cfg(feature = "server")]
crate::impl_redis!(StoredSecret, SentSecret);
#[cfg(feature = "server")]
crate::impl_sea_orm!(StoredSecret, SentSecret);

fn aead(signing_key: impl AsRef<[u8]>) -> XChaCha20Poly1305 {
    let key = Key::from_slice(signing_key.as_ref());
    XChaCha20Poly1305::new(key)
}
