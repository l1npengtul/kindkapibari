use poem_openapi::OAuthScopes;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::thread::Scope;

#[derive(
    Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, OAuthScopes,
)]
pub enum KKBScope {
    PublicRead,
    BadgesRead,
    EmailRead,
    ConnectionsRead,
    PreferencesRead,
    UserdataRead,
    ApplicationsRead,
    RecordsRead,
    OfflineRead,
    // Danger Zone
}

impl Display for KKBScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KKBScope::PublicRead => {}
            KKBScope::BadgesRead => {}
            KKBScope::EmailRead => {}
            KKBScope::ConnectionsRead => {}
            KKBScope::PreferencesRead => {}
            KKBScope::UserdataRead => {}
            KKBScope::ApplicationsRead => {}
            KKBScope::RecordsRead => {}
            KKBScope::OfflineRead => {}
        }
    }
}

impl From<KKBScope> for Scope {
    fn from(kkbscope: KKBScope) -> Self {
        todo!()
    }
}
