use kindkapibari_core::{impl_attr_err, AttrString};
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, AttrString,
)]
pub enum Roles {
    NormalUser,
    Moderator,
    Administrator,
}

impl_attr_err!(Roles);
