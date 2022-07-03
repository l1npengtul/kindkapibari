use crate::{gender::Gender, pronouns::Pronouns};
use chrono::{DateTime, Utc};
use language_tags::LanguageTag;
use std::{
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub const CURRENT_SCHEMA: u64 = 0;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserData {
    // pub schema: u64,
    pub gender: Gender,
    pub pronouns: Pronouns,
    pub birthday: Option<DateTime<Utc>>,
    pub locale: Locale,
}

impl UserData {
    #[must_use]
    pub fn new(
        gender: Gender,
        pronouns: Pronouns,
        birthday: Option<DateTime<Utc>>,
        locale: Locale,
    ) -> Self {
        Self {
            // schema: CURRENT_SCHEMA,
            gender,
            pronouns,
            birthday,
            locale,
        }
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            gender: Gender::default(),
            pronouns: Pronouns::default(),
            birthday: Option::from(Utc::now()),
            locale: LanguageTag::parse("en").unwrap().into(), // Panics: This is a valid locale and thus shouldn't crash.
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Locale {
    lang_tag: LanguageTag,
}

impl Deref for Locale {
    type Target = LanguageTag;

    fn deref(&self) -> &Self::Target {
        &self.lang_tag
    }
}

impl DerefMut for Locale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lang_tag
    }
}

impl FromStr for Locale {
    type Err = language_tags::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lang_tag: LanguageTag::from_str(s)?,
        })
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.lang_tag)
    }
}

impl From<LanguageTag> for Locale {
    fn from(lt: LanguageTag) -> Self {
        Self { lang_tag: lt }
    }
}

impl From<Locale> for LanguageTag {
    fn from(lc: Locale) -> Self {
        lc.lang_tag
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
pub struct UserSignupRequest {
    pub username: String,
    pub email: String,
    pub pfp: String,
    pub other_data: UserData,
}

#[cfg(feature = "server")]
crate::impl_redis!(UserData, UserSignupRequest, Locale);
#[cfg(feature = "server")]
crate::impl_sea_orm!(UserData, UserSignupRequest, Locale);
