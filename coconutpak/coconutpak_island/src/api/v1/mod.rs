use crate::access::login::{
    verify_apikey, verify_session, API_KEY_PREFIX_NO_DASH, TOKEN_PREFIX_NO_DASH,
};
use crate::{schema, schema::user::Model, AppData};
use kindkapibari_core::motd::MessageOfTheDay;
use kindkapibari_core::secret::decode_gotten_secret;
use poem::web::Json;
use poem::{web::Data, Request};
use poem_openapi::{auth::ApiKey, SecurityScheme};
use std::sync::Arc;

pub mod coconutpak;
pub mod user;

#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "CIAuth",
    in = "cookie",
    checker = "coconutpak_auth_checker"
)]
pub struct CoconutPakUserAuthentication(schema::user::Model);

async fn coconutpak_auth_checker(
    data: Data<Arc<AppData>>,
    _: &Request,
    key: ApiKey,
) -> Option<Model> {
    let decoded = decode_gotten_secret(
        key.key,
        ":",
        data.config.read().await.signing_key.as_bytes(),
    )
    .ok()?;
    match decoded.secret_type.as_str() {
        API_KEY_PREFIX_NO_DASH => verify_apikey(data.0, decoded.hash, decoded.salt)
            .ok()
            .flatten(),
        TOKEN_PREFIX_NO_DASH => verify_session(data.0, decoded.hash, decoded.salt)
            .ok()
            .flatten(),
        _ => return None,
    }
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
