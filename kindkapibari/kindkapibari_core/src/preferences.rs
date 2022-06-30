use std::collections::HashMap;

const V1: u32 = 0;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub version: u32,
    pub coconutpak_settings: CoconutPakIslandSettings,
    pub appearance: Appearance,
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

#[cfg(feature = "server")]
crate::impl_redis!(Preferences, Appearance, BackGroundColour, FontSize);
#[cfg(feature = "server")]
crate::impl_sea_orm!(Preferences, Appearance, BackGroundColour, FontSize);
