use crate::{gender::Gender, pronouns::Pronouns};
use chrono::{DateTime, Utc};
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub const CURRENT_SCHEMA: u64 = 0;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
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

    #[must_use]
    pub fn verify(&self) -> bool {
        let gender = match &self.gender {
            Gender::Man | Gender::Woman | Gender::NonBinary => true,
            Gender::Custom(c) => c.len() > 30,
        };

        let pronoun = match &self.pronouns {
            Pronouns::Custom(c) => c.verify(),
            _ => true,
        };

        let date = self.birthday.unwrap_or_else(Utc::now) > Utc::now();

        gender && pronoun && date
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
#[cfg_attr(feature = "server", derive(utoipa::Component))]
#[cfg_attr(feature = "server", component(value_type = String, default = locale_default))]
pub struct Locale(LanguageTag);

fn locale_default() -> String {
    "en-US".to_string()
}

impl Deref for Locale {
    type Target = LanguageTag;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Locale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Locale {
    type Err = language_tags::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(LanguageTag::from_str(s)?))
    }
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.as_str())
    }
}

impl From<LanguageTag> for Locale {
    fn from(lt: LanguageTag) -> Self {
        Self(lt)
    }
}

impl From<Locale> for LanguageTag {
    fn from(lc: Locale) -> Self {
        lc.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
pub struct UserSignupRequest {
    pub username: String,
    pub email: String,
    pub profile_picture: String,
    pub other_data: UserData,
}

#[cfg(feature = "server")]
crate::impl_redis!(UserData, UserSignupRequest, Locale);
#[cfg(feature = "server")]
crate::impl_sea_orm!(UserData, UserSignupRequest, Locale);
