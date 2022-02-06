use crate::preferences::Theme;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct User {
    user_id: Uuid,
}
