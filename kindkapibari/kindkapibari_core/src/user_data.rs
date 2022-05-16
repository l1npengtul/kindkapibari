use crate::{gender::Gender, pronouns::Pronouns};
use chrono::{Date, Utc};
use language_tags::LanguageTag;
#[cfg(feature = "server")]
use sea_orm::{DbErr, QueryResult, TryGetError, TryGetable};

const CURRENT_SCHEMA: u64 = 0;

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    pub schema: u64,
    pub gender: Gender,
    pub pronouns: Pronouns,
    pub birthday: Date<Utc>,
    pub locale: LanguageTag,
}

impl UserData {
    pub fn new(
        gender: Gender,
        pronouns: Pronouns,
        birthday: Date<Utc>,
        locale: LanguageTag,
    ) -> Self {
        Self {
            schema: CURRENT_SCHEMA,
            gender,
            pronouns,
            birthday,
            locale,
        }
    }
}

#[cfg(feature = "server")]
impl From<UserData> for sea_orm::Value {
    fn from(user_data: UserData) -> Self {
        sea_orm::Value::Bytes(Some(Box::new(pot::to_vec(&user_data).unwrap_or_default())))
    }
}

#[cfg(feature = "server")]
impl TryGetable for UserData {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        match pot::from_slice::<UserData>(&Vec::<u8>::try_get(res, pre, col)?) {
            Ok(user_data) => Ok(user_data),
            Err(why) => Err(TryGetError::DbErr(DbErr::Custom(why.to_string()))),
        }
    }
}
