use kindkapibari_proc::AttrString;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
pub enum Role {
    NewUser,
    NormalUser,
    Moderator,
    Server,
    Administrator,
}

impl Default for Role {
    fn default() -> Self {
        Role::NormalUser
    }
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(Role);
