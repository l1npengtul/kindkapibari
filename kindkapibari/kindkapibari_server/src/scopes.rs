use poem_openapi::OAuthScopes;
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, OAuthScopes,
)]
pub enum Scope {
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
    // Only KindKapiBari itself can request this scope.
    KKBApp,
}
