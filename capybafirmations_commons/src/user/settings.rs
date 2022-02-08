use crate::preferences::Preferences;

use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable, Queryable))]
#[cfg_attr(feature = "server", table_name = "user_settings")]
pub struct UserSettings {
    pub id: Uuid,
    pub preferences: Preferences,
    pub paks: Vec<Uuid>,
}

#[cfg(feature = "server")]
table! {
    user_settings {
        id -> Uuid,
        preferences -> Jsonb,
        paks -> Array<Uuid>,
    }
}
