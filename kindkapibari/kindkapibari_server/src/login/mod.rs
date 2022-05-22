mod login;
mod oauth;
mod oauth_thirdparty;

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
use tracing::instrument;

use crate::access::{
    auth::login::LOGIN_TOKEN_PREFIX_NO_DASH, auth::oauth::OAUTH_ACCESS_PREFIX_NO_DASH,
    TOKEN_SEPERATOR,
};

pub struct AuthorizedUser {
    pub user: user::Model,
    pub oauth_scopes: Vec<Scope>,
    pub roles: Vec<Roles>,
}

// it ""works""
// use the same auth fn for all login tokens and oauth tokens
#[derive(SecurityScheme)]
#[oai(
    type = "bearer",
    bearer_format = "`nonce`.`type`.salt-hash"
    checker = "authorization_checker"
)]
pub struct KKBUserAuthorization(AuthorizedUser);

#[instrument]
async fn authorization_checker(_req: &Request, bearer: &Bearer) -> Option<AuthorizedUser> {
    None
}
