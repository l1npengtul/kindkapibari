use crate::roles::Roles;
use crate::schema::{users::*, *};
use crate::scopes::Scope;
use poem::Request;
use poem_openapi::auth::{ApiKey, Bearer};
use poem_openapi::{OAuthScopes, SecurityScheme};
use serde::{Deserialize, Serialize};

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

fn check_kkb_user_authorization(_: &Request, key: ApiKey) -> Option<AuthorizedUser> {
    let key = key.key;
    // decrypt the key
    // parsing the key - the key is made of 3 parts
    // {nonce}.{front}.{payload}
    let splitted = key.split(".").collect::<Vec<_>>();
    if splitted.len() != 3 {
        return None;
    }
    let nonce = base64::decode(splitted[0]).ok()?;
    // this determines where the decoder pipeline goes to next
    match splitted[1] {
        "O" => {
            todo!()
        }
        "A" => {
            todo!()
        }
        _ => return None;
    }
}
