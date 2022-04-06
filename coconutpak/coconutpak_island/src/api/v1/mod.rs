use crate::login::{verify_apikey, verify_session};
use crate::schema::user::Model;
use crate::{AppData, ARGON2};
use argon2::{Algorithm, Argon2, Params, Version};
use kindkapibari_core::motd::MessageOfTheDay;
use poem::web::{Data, Json};
use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, SecurityScheme};
use redis::Cmd;
use std::sync::Arc;

pub mod coconutpak;
pub mod login;
pub mod user;

const AUTH_REDIS_KEY_START: [u8; 16] = *b"coconutpak:auth:";

#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "X-API-Key",
    in = "header",
    checker = "coconutpak_auth_checker"
)]
pub struct CoconutPakUserApiKey(pub Model);

async fn coconutpak_auth_checker(
    data: Data<Arc<AppData>>,
    request: &Request,
    key: ApiKey,
) -> Option<Model> {
    // session
    if key.key.is_empty() {
        let auth_token = request.cookie().get("authtoken")?.value_str().to_string();
        if !auth_token.is_empty() {
            return verify_session(data.clone(), auth_token);
        }
    } else {
        return verify_apikey(data.clone(), key.key);
    }
    return None;
}

struct Api;

#[OpenApi(prefix_path = "/v1", tag = "super::VersionTags::V1")]
impl Api {
    #[oai(path = "motd", method = "get")]
    async fn motd(&self) -> Json<MessageOfTheDay> {}
}
