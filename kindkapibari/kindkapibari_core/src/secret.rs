use crate::roles::Role;
use chrono::{Duration, TimeZone, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::ops::Add;
use utoipa::Component;

#[derive(Copy, Clone, Debug, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Component))]
pub enum TokenType {
    Login,
    OAuth,
    Game,
    Custom,
}

#[derive(Copy, Clone, Debug, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Component))]
pub struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub jti: u64,
    pub user_id: u64,
    pub role: Role,
    pub machine_id: u8,
    pub token_type: TokenType,
}

impl TokenClaims {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn set_id(mut self, id: u64) -> Self {
        self.jti = id;
        self
    }

    #[must_use]
    pub fn set_user(mut self, id: u64) -> Self {
        self.user_id = id;
        self
    }

    #[must_use]
    pub fn set_role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    #[must_use]
    pub fn set_machine_id(mut self, id: u8) -> Self {
        self.machine_id = id;
        self
    }

    #[must_use]
    pub fn set_token_type(mut self, tt: TokenType) -> Self {
        self.token_type = tt;
        self
    }
}

impl Default for TokenClaims {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn default() -> Self {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = now.add(Duration::seconds(420)).timestamp() as usize;
        Self {
            exp,
            iat,
            jti: 0,
            user_id: 0,
            role: Role::default(),
            machine_id: 0,
            token_type: TokenType::Login,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Component))]
pub struct RefreshClaims {
    pub exp: usize,
    pub iat: usize,
    pub jti: u64,
    pub reference_token: u64,
    pub user_id: u64,
    pub machine_id: u8,
    pub token_type: TokenType,
}

impl RefreshClaims {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn set_id(mut self, id: u64) -> Self {
        self.jti = id;
        self
    }

    #[must_use]
    pub fn set_user(mut self, id: u64) -> Self {
        self.user_id = id;
        self
    }

    #[must_use]
    pub fn set_reference_token(mut self, id: u64) -> Self {
        self.reference_token = id;
        self
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn set_expire(mut self, duration: Duration) -> Self {
        let duration = duration.num_seconds() as usize;
        self.exp += duration;
        self
    }

    #[must_use]
    pub fn set_machine_id(mut self, id: u8) -> Self {
        self.machine_id = id;
        self
    }

    #[must_use]
    pub fn set_token_type(mut self, tt: TokenType) -> Self {
        self.token_type = tt;
        self
    }
}

impl Default for RefreshClaims {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn default() -> Self {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = now.add(Duration::days(7)).timestamp() as usize;
        Self {
            exp,
            iat,
            jti: 0,
            reference_token: 0,
            user_id: 0,
            machine_id: 0,
            token_type: TokenType::Login,
        }
    }
}

impl From<TokenClaims> for RefreshClaims {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn from(tc: TokenClaims) -> Self {
        let gen_then = Utc
            .timestamp(tc.iat as i64, 0)
            .add(Duration::days(7))
            .timestamp() as usize;
        Self {
            exp: gen_then,
            iat: tc.iat,
            jti: 0,
            reference_token: tc.jti,
            user_id: tc.user_id,
            machine_id: tc.machine_id,
            token_type: tc.token_type,
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Component))]
pub struct JWTPair {
    pub access: String,
    pub refresh: String,
}

pub fn create_new_token(
    claims: &TokenClaims,
    signing: impl AsRef<[u8]>,
) -> Result<String, Box<dyn std::error::Error>> {
    let access_jwt = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(signing.as_ref()),
    )?;
    Ok(access_jwt)
}

pub fn create_new_token_with_refresh(
    claims: &TokenClaims,
    refresh: &RefreshClaims,
    signing: impl AsRef<[u8]>,
) -> Result<JWTPair, Box<dyn std::error::Error>> {
    let access = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(signing.as_ref()),
    )?;
    let refresh = encode(
        &Header::default(),
        &refresh,
        &EncodingKey::from_secret(signing.as_ref()),
    )?;
    Ok(JWTPair { access, refresh })
}

pub fn decode_access_token(
    token: impl AsRef<str>,
    signing: impl AsRef<[u8]>,
) -> Result<TokenClaims, Box<dyn std::error::Error>> {
    let mut validation = Validation::default();
    validation.leeway = 10;
    let token = decode::<TokenClaims>(
        token.as_ref(),
        &DecodingKey::from_secret(signing.as_ref()),
        &validation,
    )?;
    Ok(token.claims)
}

pub fn decode_access_token_without_time_verification(
    token: impl AsRef<str>,
    signing: impl AsRef<[u8]>,
) -> Result<TokenClaims, Box<dyn std::error::Error>> {
    let mut validation = Validation::default();
    validation.validate_exp = false;
    let token = decode::<TokenClaims>(
        token.as_ref(),
        &DecodingKey::from_secret(signing.as_ref()),
        &validation,
    )?;
    Ok(token.claims)
}

pub fn decode_refresh_token(
    token: impl AsRef<str>,
    signing: impl AsRef<[u8]>,
) -> Result<RefreshClaims, Box<dyn std::error::Error>> {
    let validation = Validation::default();
    let token = decode::<RefreshClaims>(
        token.as_ref(),
        &DecodingKey::from_secret(signing.as_ref()),
        &validation,
    )?;
    Ok(token.claims)
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(TokenClaims, RefreshClaims);
#[cfg(feature = "server")]
crate::impl_redis!(TokenClaims, RefreshClaims);
