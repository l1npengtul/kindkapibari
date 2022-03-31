use argon2::{Algorithm, Argon2, Params, Version};
use crate::schema::user::Model;
use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::{OpenApi, SecurityScheme};
use redis::Cmd;
use crate::ARGON2;

pub mod coconutpak;
pub mod login;
pub mod user;

const AUTH_REDIS_KEY_START: &'static [u8] = b"coconutpak:auth:";

#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "X-API-Key",
    in = "header",
    checker = "coconutpak_session_apikey_checker"
)]
pub struct CoconutPakUserApiKey(pub Model);

async fn coconutpak_session_apikey_checker(request: &Request, key: ApiKey) -> Option<Model> {
    // check if APIKEY is valid
    // check with redis
    let mut hashed_key = Vec::new();
    let prehash = (if key.key.is_empty() {
        request.cookie().get("session").unwrap_or_default().as_str()
    } else {
        &key.key
    }).into_bytes();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::default(), Params::default());
    argon2.hash_password_into(&prehash, &Vec::new(), &mut hashed_key);
    let from_redis = Cmd::get()
}

struct Api;

#[OpenApi]
impl Api {}
