use crate::roles::Roles;
use crate::schema::{users::*, *};
use crate::scopes::Scope;
use crate::AppData;
use kindkapibari_core::secret::decode_gotten_secret;
use poem::web::Data;
use poem::Request;
use poem_openapi::auth::{ApiKey, Bearer};
use poem_openapi::{OAuthScopes, SecurityScheme};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub scopes: Option<Scope>,
    pub roles: Vec<Roles>,
    pub user: user::Model,
}

// it ""works""
// use the same auth fn for all login tokens and oauth tokens
#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    in = "cookie",
    key_name = "KKBAuth",
    checker = "check_kkb_user_authorization"
)]
pub struct KKBUserAuthorization(user::Model);

async fn check_kkb_user_authorization(
    state: Data<Arc<AppData>>,
    _: &Request,
    key: ApiKey,
) -> Option<AuthorizedUser> {
    let key = key.key;
    // decrypt the key
    // parsing the key - the key is made of 3 parts
    // {nonce}.{front}.{payload}
    let key_parts =
        decode_gotten_secret(key, "-", state.config.read().await.signing_key.as_bytes()).ok()?;
    match key_parts.secret_type.as_str() {
        "OA" => {}
        "LT" => {}
        _ => None,
    }
}
