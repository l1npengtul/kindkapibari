use crate::{
    AppData,
    schema,
    schema::user::Model,
};
use kindkapibari_core::motd::MessageOfTheDay;
use poem::web::Json;
use poem::{Request, web::Data};
use poem_openapi::{auth::ApiKey, SecurityScheme};
use std::sync::Arc;
use crate::access::login::{verify_apikey, verify_session};

pub mod coconutpak;
pub mod user;

#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "X-API-Key",
    in = "header",
    checker = "api_checker"
)]
pub struct CoconutPakUserAuthentication(schema::user::Model);

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

    // #[oai(path = "/auth_required", method = "get")]
    async fn requires_auth(&self) -> bool {
        false
    }
}
