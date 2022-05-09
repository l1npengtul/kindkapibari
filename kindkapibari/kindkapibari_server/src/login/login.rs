use crate::roles::Roles;
use crate::schema::{users::*, *};
use crate::scopes::Scopes;
use poem::Request;
use poem_openapi::auth::{ApiKey, Bearer};
use poem_openapi::{OAuthScopes, SecurityScheme};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub scopes: Option<Scopes>,
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

fn check_kkb_user_authorization(_: &Request, key: ApiKey) -> Option<AuthorizedUser> {
    let key = key.key;
    // decrypt the key
}
