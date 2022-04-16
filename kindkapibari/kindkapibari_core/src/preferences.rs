#[cfg(feature = "server")]
use sea_orm::{DbErr, QueryResult, TryGetError, TryGetable, Value};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

const V1: u32 = 0;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub version: u32,
    pub coconutpak_settings: CoconutPakIslandSettings,
    pub appearance: Appearance,
}

#[cfg(feature = "server")]
impl From<Preferences> for sea_orm::Value {
    fn from(p: Preferences) -> Self {
        sea_orm::Value::Bytes(Some(Box::new(pot::to_vec(&p).unwrap_or_default())))
    }
}

#[cfg(feature = "server")]
impl TryGetable for Preferences {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        match pot::from_slice::<Preferences>(&Vec::<u8>::try_get(res, pre, col)?) {
            Ok(pref) => Ok(pref),
            Err(why) => Err(TryGetError::DbErr(DbErr::Custom(why.to_string()))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoconutPakIslandSettings {
    pub blocked_tags: Vec<String>,
    pub added_islands: HashMap<String, Option<String>>,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum FontSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum BackGroundColour {
    Day,
    Dusk,
    Night,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Appearance {
    pub font_size: FontSize,
    pub background: BackGroundColour,
    pub light: bool,
}
