use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
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
    // No external application can write to Userdata.
    ApplicationsRead,
    ApplicationsWrite,
    RecordsRead,
    OfflineRead,
}
