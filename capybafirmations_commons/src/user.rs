use crate::preferences::Theme;

pub type UserID = u128;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct User {
    user_id: u128,
}
