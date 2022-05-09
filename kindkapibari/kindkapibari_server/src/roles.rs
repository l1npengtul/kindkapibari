use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Roles {
    NormalUser,
    Moderator,
    Administrator,
}
