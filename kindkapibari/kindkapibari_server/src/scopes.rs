use poem_openapi::OAuthScopes;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, OAuthScopes,
)]
pub enum Scope {
    // This is a private user scope.
    UserReadWriteApplication,
    PublicRead,
    PublicWrite,
    BadgesRead,
    EmailRead,
    EmailWrite,
    ConnectionsRead,
    ConnectionsWrite,
    PreferencesRead,
    PreferencesWrite,
    UserdataDecryptRead,
    ApplicationsRead,
    ApplicationsWrite,
    RecordsRead,
    OfflineRead,
    // Application (Read Only)
    Application,
}
