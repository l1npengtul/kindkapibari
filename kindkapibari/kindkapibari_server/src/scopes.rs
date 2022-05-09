use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Scopes {
    PublicRead,
    PublicWrite,
    BadgesRead,
    EmailRead,
    EmailWrite,
    ConnectionsRead,
    ConnectionsWrite,
    PreferencesRead,
    PreferencesWrite,
    UserdataRead,
    UserdataWrite,
    ApplicationsRead,
    ApplicationsWrite,
    RecordsRead,
    OfflineRead,
}
