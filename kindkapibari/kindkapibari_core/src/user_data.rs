use crate::language::Language;
use crate::{gender::Gender, pronouns::Pronouns};
use chrono::{DateTime, Utc};
use language_tags::LanguageTag;
#[cfg(feature = "server")]
use sea_orm::{DbErr, QueryResult, TryGetError, TryGetable};

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    pub username: String,
    pub handle: String,
    pub gender: Gender,
    pub pronouns: Pronouns,
    pub birthday: Option<DateTime<Utc>>,
    pub registered_date: DateTime<Utc>,
    pub language: LanguageTag,
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
