use kindkapibari_proc::AttrString;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString)]
pub enum Badge {
    AlphaTester(u64),
    EarlyAdopter(u64), // UTC Timestamp of join date
    Verified,
    Developer,
    Contributor,
    Supporter,
    PengChanApproved,
}

#[derive(Clone, Ord, Debug, Hash, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Badges {
    int: Vec<Badge>,
}

impl Deref for Badges {
    type Target = Vec<Badge>;

    fn deref(&self) -> &Self::Target {
        &self.int
    }
}

impl DerefMut for Badges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.int
    }
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(Badges, Badge);
