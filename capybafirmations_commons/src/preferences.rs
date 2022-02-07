use crate::pronouns::Pronouns;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub theme: Theme,
    pub pronouns: Pronouns,
    pub subscribed_paks: Vec<Uuid>,
}
