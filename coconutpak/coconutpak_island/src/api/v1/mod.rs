use crate::login::{verify_apikey, verify_session};
use crate::schema::user::Model;
use crate::{AppData, ARGON2};
use argon2::{Algorithm, Argon2, Params, Version};
use chrono::Locale::da_DK;
use kindkapibari_core::motd::MessageOfTheDay;
use poem::web::{Data, Json};
use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, SecurityScheme};
use redis::Cmd;
use std::sync::Arc;

pub mod coconutpak;
pub mod user;

#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "X-API-Key",
    in = "header",
    checker = "coconutpak_auth_checker"
)]
pub struct CoconutPakUserAuthentication(pub Model);

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

struct Api {
    data: Arc<AppData>,
}

// #[OpenApi(prefix_path = "/v1", tag = "super::VersionTags::V1")]
impl Api {
    // #[oai(path = "/motd", method = "get")]
    async fn motd(&self) -> Json<MessageOfTheDay> {
        // TODO: replace with webconsole
        Json(MessageOfTheDay {
            color: "red".to_string(),
            text: "hi".to_string(),
            has_button: false,
            button_label: None,
            button_link: None,
        })
    }

    // #[oai(path = "/readonly", method = "get")]
    async fn read_only(&self) -> bool {
        false
    }

    // #[oai(path = "/kkb_auth_supported", method = "get")]
    async fn supports_kkb_auth(&self) -> bool {
        self.data.config.read().await.support_official_kkb_login
    }
}
