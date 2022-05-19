mod oauth_thirdparty;
mod oauth;

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
    pub roles: Vec<Role>,
}

// it ""works""
// use the same auth fn for all login tokens and oauth tokens
#[derive(SecurityScheme)]
#[cfg_attr(
    debug_assertions,
    oai(
        type = "oauth2",
        flows(authorization_code(
            authorization_url = "http://localhost:3003/authorize",
            token_url = "http://localhost:3003/token",
            scopes = "crate::scopes::Scope"
        ))
    )
)]
#[cfg_attr(
    not(debug_assertions),
    oai(
        type = "oauth2",
        flows(authorization_code(
            authorization_url = "https://kindkapibari.land/authorize",
            token_url = "https://kindkapibari.land/authorize",
            scopes = "crate::scopes::Scope"
        ))
    )
)]
pub struct KKBUserAuthorization(AuthorizedUser);
