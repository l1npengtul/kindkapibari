use kindkapibari_proc::AttrString;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString)]
pub enum Role {
    NewUser,
    NormalUser,
    Moderator,
    Server,
    Administrator,
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(Role);
