use crate::KKBScope;
use kindkapibari_core::impl_redis;
use serde::{Deserialize, Serialize};

pub mod badges;
pub mod connections;
pub mod login_tokens;
pub mod oauth_authorizations;
pub mod passwords;
pub mod preferences;
pub mod user;
pub mod userdata;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub scopes: Vec<KKBScope>,
    pub user: user::Model,
}

impl_redis!(AuthorizedUser);
