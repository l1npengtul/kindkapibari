use kindkapibari_proc::AttrString;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString)]
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

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KKBScopes {
    int: Vec<KKBScope>,
}

impl Deref for KKBScopes {
    type Target = Vec<KKBScope>;

    fn deref(&self) -> &Self::Target {
        &self.int
    }
}

impl DerefMut for KKBScopes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.int
    }
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(KKBScopes, KKBScope);
