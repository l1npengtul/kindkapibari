use crate::{gender::Gender, pronouns::Pronouns, reminder::Reminders, sober::Sobers};
use chrono::{Date, Utc};
use language_tags::LanguageTag;

const CURRENT_SCHEMA: u64 = 0;

#[derive(Clone, Debug, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    // pub schema: u64,
    pub gender: Gender,
    pub pronouns: Pronouns,
    pub birthday: Date<Utc>,
    pub locale: LanguageTag,
    pub sobers: Sobers,
    pub reminders: Reminders,
}

impl UserData {
    pub fn new(
        gender: Gender,
        pronouns: Pronouns,
        birthday: Date<Utc>,
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

#[cfg(feature = "server")]
crate::impl_sea_orm!(UserData);
