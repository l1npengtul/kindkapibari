use kindkapibari_core::{impl_redis, scopes::KKBScope};
use serde::{Deserialize, Serialize};

pub mod badges;
pub mod connections;
pub mod oauth_authorizations;
pub mod onetime_reminders;
pub mod passwords;
pub mod preferences;
pub mod recurring_reminders;
pub mod refresh_tokens;
pub mod sobers;
pub mod statistics;
pub mod user;
pub mod userdata;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizedUser {
    pub scopes: Vec<KKBScope>,
    pub user: user::Model,
}

impl_redis!(AuthorizedUser);
