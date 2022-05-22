use poem_openapi::OAuthScopes;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, OAuthScopes,
)]
pub enum Scope {
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
