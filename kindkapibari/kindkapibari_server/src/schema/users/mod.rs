use crate::Scope;
use serde::{Deserialize, Serialize};

pub mod badges;
pub mod connections;
pub mod login_tokens;
pub mod oauth_authorizations;
pub mod passwords;
pub mod preferences;
pub mod user;
pub mod userdata;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub scopes: Option<Scope>,
    pub user: user::Model,
}
