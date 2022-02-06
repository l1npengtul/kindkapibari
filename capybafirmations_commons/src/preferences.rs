use crate::pronouns::Pronouns;
#[cfg(feature = "server")]
use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    theme: Theme,
    pronouns: Pronouns,
}
