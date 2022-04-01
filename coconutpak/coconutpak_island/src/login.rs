use crate::schema::user;
use poem::Request;
use poem_openapi::auth::ApiKey;

const AUTH_REDIS_KEY_START: [u8; 16] = *b"coconutpak:auth:";

pub async fn generate_apikey() -> (String, Vec<u8>) {}

pub async fn generate_session() -> (String, Vec<u8>) {}

pub async fn verify_apikey(request: &Request, api_key: ApiKey) -> Option<user::Model> {}

pub async fn verify_session(request: &Request, api_key: ApiKey) -> Option<user::Model> {}
