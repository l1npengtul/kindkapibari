use crate::preferences::Preferences;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct UserSettings {
    pub user_id: Uuid,
    pub preferences: Preferences,
}
