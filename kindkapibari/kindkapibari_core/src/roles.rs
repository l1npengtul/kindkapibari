use kindkapibari_proc::AttrString;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString)]
pub enum Roles {
    NormalUser,
    Moderator,
    Administrator,
}
