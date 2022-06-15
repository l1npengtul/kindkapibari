use crate::{gender::Gender, pronouns::Pronouns, reminder::Reminders, sober::Sobers};
use chrono::{DateTime, Utc};
use language_tags::LanguageTag;

const CURRENT_SCHEMA: u64 = 0;

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    // pub schema: u64,
    pub gender: Gender,
    pub pronouns: Pronouns,
    pub birthday: DateTime<Utc>,
    pub locale: LanguageTag,
    pub sobers: Sobers,
    pub reminders: Reminders,
}

impl UserData {
    pub fn new(
        gender: Gender,
        pronouns: Pronouns,
        birthday: DateTime<Utc>,
        locale: LanguageTag,
    ) -> Self {
        Self {
            // schema: CURRENT_SCHEMA,
            gender,
            pronouns,
            birthday,
            locale,
            ..Default::default()
        }
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            gender: Default::default(),
            pronouns: Default::default(),
            birthday: Utc::now(),
            locale: LanguageTag::parse("en").unwrap(), // Panics: This is a valid locale and thus shouldn't crash.
            sobers: Default::default(),
            reminders: Default::default(),
        }
    }
}

#[cfg(feature = "server")]
crate::impl_sea_orm!(UserData);
