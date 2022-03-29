use crate::schema::user::Model;
use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::{OpenApi, SecurityScheme};

pub mod coconutpak;
pub mod login;
pub mod user;

#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "X-API-Key",
    in = "header",
    checker = "coconutpak_session_apikey_checker"
)]
pub struct CoconutPakUserApiKey(pub Model);

async fn coconutpak_session_apikey_checker(request: &Request, key: ApiKey) -> Option<Model> {}

struct Api;

#[OpenApi]
impl Api {}
