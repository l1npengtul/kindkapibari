use crate::{
    error::KKBCoreError,
    gender::{Gender, OnlyThree},
    pronouns::PronounProfile,
    user_data::UserData,
};
use chrono::{DateTime, Utc, MIN_DATETIME};
use language_tags::LanguageTag;
use tera::{Context, Tera};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TemplateUserData {
    pronouns: PronounProfile,
    gender: Gender,
    gender_three: OnlyThree,
    username: String,
    birthday: DateTime<Utc>, // WHAT THE FUCK WHYYYYYY WHY DOES NOT `Date<Utc>` IMPLEMENT SERIALIZE AND DESERIALIZE???????????
    registerdate: DateTime<Utc>, // WHAT THE FUCK IS WRONG WITH CHRONO?????? WHAHTWAHT(*WAHTUIOWAHOTIWAIOTJOIPWA
    langtag: LanguageTag,
}

impl From<UserData> for TemplateUserData {
    fn from(ud: UserData) -> Self {
        TemplateUserData {
            pronouns: ud.pronouns.as_profile(),
            gender: ud.gender,
            gender_three: ud.gender.into(),
            username: ud.username,
            birthday: ud.birthday.unwrap_or(MIN_DATETIME),
            registerdate: ud.registered_date,
            langtag: ud.language,
        }
    }
}

impl From<&UserData> for TemplateUserData {
    fn from(ud: &UserData) -> Self {
        let ud = ud.clone(); // lazy peng again ;-;
        TemplateUserData {
            pronouns: ud.pronouns.as_profile(),
            gender: ud.gender,
            gender_three: ud.gender.into(),
            username: ud.username,
            birthday: ud.birthday.unwrap_or(MIN_DATETIME),
            registerdate: ud.registered_date,
            langtag: ud.preferred_language,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Templater {
    tera: Tera,
    context: Context,
}

impl Templater {
    pub fn new(user_data: &UserData) -> Result<Templater, KKBCoreError> {
        let tera = Tera::default();
        let context = Context::from_value(
            serde_json::to_value(TemplateUserData::from(user_data)).unwrap_or_default(),
        )?;
        Ok(Templater { tera, context })
    }
}
