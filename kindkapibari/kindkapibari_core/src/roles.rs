use kindkapibari_proc::AttrString;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

const ORDER: [Role; 7] = [
    Role::NewUser,
    Role::NormalUser,
    Role::Supporter,
    Role::Verified,
    Role::Moderator,
    Role::Server,
    Role::Administrator,
];

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, AttrString)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
pub enum Role {
    NewUser,
    NormalUser,
    Supporter,
    Verified,
    Moderator,
    Server,
    Administrator,
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let index_self = ORDER.iter().position(|x| x == self)?;
        let index_other = ORDER.iter().position(|x| x == other)?;

        index_self.partial_cmp(&index_other)
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::NormalUser
    }
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(Role);
