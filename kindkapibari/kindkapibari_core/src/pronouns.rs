use sea_orm::sea_query::{ColumnType, ValueType, ValueTypeErr};
use sea_orm::Value;
#[cfg(feature = "server")]
use sea_orm::{DbErr, QueryResult, TryGetError, TryGetable};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Pronouns {
    HeHim,
    SheHer,
    TheyThem,
    PerPers,
    ItIts,
    FaeFaer,
    XeXyrs,
    ZeZie,
    AeAers,
    Custom(PronounProfile),
}

impl Pronouns {
    pub fn decode<S: AsRef<str>>(nouns: S) -> Option<PronounProfile> {
        match nouns.as_ref().to_lowercase().as_ref() {
            "he/him" => Some(PronounProfile::new("he", "him", "his", "his", "himself")),
            "she/her" => Some(PronounProfile::new("she", "her", "hers", "her", "herself")),
            "they/them" => Some(PronounProfile::new(
                "they", "them", "theirs", "their", "themself",
            )),
            "per/pers" => Some(PronounProfile::new("per", "per", "per", "pers", "perself")),
            "it/its" => Some(PronounProfile::new("it", "it", "its", "its", "itself")),
            "fae/faer" | "fae/faers" => Some(PronounProfile::new(
                "fae", "faer", "faer", "faers", "faerself",
            )),
            "xe" | "xe/xyrs" | "xe/xers" | "xe/xem" => {
                Some(PronounProfile::new("xe", "xem", "xyr", "xyrs", "xemself"))
            }
            "ze" | "ze/zie" | "ze/zers" | "ze/zirs" => {
                Some(PronounProfile::new("ze", "zir", "zir", "zirs", "zirself"))
            }
            "ae/aers" => Some(PronounProfile::new("ae", "aer", "aer", "aers", "aerself")),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_profile(&self) -> PronounProfile {
        match self {
            Pronouns::HeHim => PronounProfile::new("he", "him", "his", "his", "himself"),
            Pronouns::SheHer => PronounProfile::new("she", "her", "hers", "her", "herself"),
            Pronouns::TheyThem => {
                PronounProfile::new("they", "them", "theirs", "their", "themself")
            }
            Pronouns::PerPers => PronounProfile::new("per", "per", "per", "pers", "perself"),
            Pronouns::ItIts => PronounProfile::new("it", "it", "its", "its", "itself"),
            Pronouns::FaeFaer => PronounProfile::new("fae", "faer", "faer", "faers", "faerself"),
            Pronouns::XeXyrs => PronounProfile::new("xe", "xem", "xyr", "xyrs", "xemself"),
            Pronouns::ZeZie => PronounProfile::new("ze", "zir", "zir", "zirs", "zirself"),
            Pronouns::AeAers => PronounProfile::new("ae", "aer", "aer", "aers", "aerself"),
            Pronouns::Custom(c) => c.clone(),
        }
    }
}

impl From<PronounProfile> for Pronouns {
    fn from(pp: PronounProfile) -> Self {
        let pps = PronounProfileStr::from(&pp);
        match pps {
            PronounProfileStr::HE_HIM => Pronouns::HeHim,
            PronounProfileStr::SHE_HER => Pronouns::SheHer,
            PronounProfileStr::THEY_THEM => Pronouns::TheyThem,
            PronounProfileStr::PER_PERS => Pronouns::PerPers,
            PronounProfileStr::IT_ITS => Pronouns::ItIts,
            PronounProfileStr::FAE_FAER => Pronouns::FaeFaer,
            PronounProfileStr::XE_XYRS => Pronouns::XeXyrs,
            PronounProfileStr::ZE_ZIE => Pronouns::ZeZie,
            PronounProfileStr::AE_AERS => Pronouns::AeAers,
            profile => Pronouns::Custom(profile.into()),
        }
    }
}

#[cfg(feature = "server")]
impl TryGetable for Pronouns {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        match pot::from_slice::<PronounProfile>(&Vec::<u8>::try_get(res, pre, col)?) {
            Ok(pp) => Ok(Pronouns::from(pp)),
            Err(why) => Err(TryGetError::DbErr(DbErr::Custom(why.to_string()))),
        }
    }
}
#[cfg(feature = "server")]
impl From<Pronouns> for sea_orm::Value {
    fn from(p: Pronouns) -> Self {
        sea_orm::Value::Bytes(Some(Box::new(pot::to_vec(&p).unwrap_or_default())))
    }
}

#[cfg(feature = "server")]
impl ValueType for Pronouns {
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        match v {
            Value::Bytes(Some(bytes)) => pot::from_slice::<Self>(&bytes).map_err(|_| ValueTypeErr),
            _ => Err(ValueTypeErr),
        }
    }

    fn type_name() -> String {
        stringify!(Pronouns).to_string()
    }

    fn column_type() -> ColumnType {
        ColumnType::Binary(None)
    }
}

#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct PronounProfile {
    pub(crate) nominative: String,
    pub(crate) accusative: String,
    pub(crate) pronominal: String,
    pub(crate) predicative: String,
    pub(crate) reflexive: String,
}

impl PronounProfile {
    pub fn new<S: AsRef<str>>(
        nominative: S,
        accusative: S,
        pronominal: S,
        predicative: S,
        reflexive: S,
    ) -> Self {
        PronounProfile {
            nominative: nominative.as_ref().to_string(),
            accusative: accusative.as_ref().to_string(),
            pronominal: pronominal.as_ref().to_string(),
            predicative: predicative.as_ref().to_string(),
            reflexive: reflexive.as_ref().to_string(),
        }
    }

    pub fn set_nominative(&mut self, nominative: String) {
        self.nominative = nominative;
    }
    pub fn set_accusative(&mut self, accusative: String) {
        self.accusative = accusative;
    }
    pub fn set_pronominal(&mut self, pronominal: String) {
        self.pronominal = pronominal;
    }
    pub fn set_predicative(&mut self, predicative: String) {
        self.predicative = predicative;
    }
    pub fn set_reflexive(&mut self, reflexive: String) {
        self.reflexive = reflexive;
    }

    #[must_use]
    pub fn nominative(&self) -> &str {
        &self.nominative
    }
    #[must_use]
    pub fn accusative(&self) -> &str {
        &self.accusative
    }
    #[must_use]
    pub fn pronominal(&self) -> &str {
        &self.pronominal
    }
    #[must_use]
    pub fn predicative(&self) -> &str {
        &self.predicative
    }
    #[must_use]
    pub fn reflexive(&self) -> &str {
        &self.reflexive
    }
}

#[cfg(feature = "server")]
impl TryGetable for PronounProfile {
    fn try_get(res: &QueryResult, pre: &str, col: &str) -> Result<Self, TryGetError> {
        match pot::from_slice::<PronounProfile>(&Vec::<u8>::try_get(res, pre, col)?) {
            Ok(pp) => Ok(pp),
            Err(why) => Err(TryGetError::DbErr(DbErr::Custom(why.to_string()))),
        }
    }
}

#[cfg(feature = "server")]
impl From<PronounProfile> for sea_orm::Value {
    fn from(pp: PronounProfile) -> Self {
        sea_orm::Value::Bytes(Some(Box::new(pot::to_vec(&pp).unwrap_or_default())))
    }
}

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct PronounProfileStr<'a> {
    pub(crate) nominative: &'a str,
    pub(crate) accusative: &'a str,
    pub(crate) pronominal: &'a str,
    pub(crate) predicative: &'a str,
    pub(crate) reflexive: &'a str,
}

impl<'a> PronounProfileStr<'a> {
    pub const HE_HIM: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "he",
        accusative: "him",
        pronominal: "his",
        predicative: "his",
        reflexive: "himself",
    };
    pub const SHE_HER: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "she",
        accusative: "her",
        pronominal: "hers",
        predicative: "her",
        reflexive: "herself",
    };
    pub const THEY_THEM: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "they",
        accusative: "them",
        pronominal: "theirs",
        predicative: "their",
        reflexive: "themself",
    };
    pub const PER_PERS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "per",
        accusative: "per",
        pronominal: "per",
        predicative: "pers",
        reflexive: "perself",
    };
    pub const IT_ITS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "it",
        accusative: "it",
        pronominal: "its",
        predicative: "its",
        reflexive: "itself",
    };
    pub const FAE_FAER: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "fae",
        accusative: "faer",
        pronominal: "faer",
        predicative: "faers",
        reflexive: "faerself",
    };
    pub const XE_XYRS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "xe",
        accusative: "xem",
        pronominal: "xyr",
        predicative: "xyrs",
        reflexive: "xemself",
    };
    pub const ZE_ZIE: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "ze",
        accusative: "zir",
        pronominal: "zir",
        predicative: "zirs",
        reflexive: "zirself",
    };
    pub const AE_AERS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: "ae",
        accusative: "aer",
        pronominal: "aer",
        predicative: "aers",
        reflexive: "aerself",
    };
}

impl<'a> From<&'a PronounProfile> for PronounProfileStr<'a> {
    fn from(pp: &'a PronounProfile) -> Self {
        PronounProfileStr {
            nominative: pp.nominative.as_str(),
            accusative: pp.accusative.as_str(),
            pronominal: pp.pronominal.as_str(),
            predicative: pp.predicative.as_str(),
            reflexive: pp.reflexive.as_str(),
        }
    }
}

impl<'a> From<PronounProfileStr<'a>> for PronounProfile {
    fn from(pps: PronounProfileStr<'a>) -> Self {
        PronounProfile {
            nominative: pps.nominative.to_string(),
            accusative: pps.accusative.to_string(),
            pronominal: pps.pronominal.to_string(),
            predicative: pps.predicative.to_string(),
            reflexive: pps.reflexive.to_string(),
        }
    }
}