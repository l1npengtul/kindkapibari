use chrono::Utc;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
#[cfg(feature = "server")]
use sea_orm::{
    sea_query::{ColumnType, ValueType, ValueTypeErr},
    DbErr, QueryResult, TryGetError, TryGetable, Value,
};
use std::borrow::Cow;

// TODO: Localized Pronouns! This is English centric!
// EX: some languages don't have gender neutral pronouns or neopronouns. We should support that!
// mfw ill have to translate this later AATAGAGOAIWGUAWHUIGHIAHGWAGUAWU

pub const PRONOUNS_CONST_BUILTIN: [PronounProfileStr<'static>; 9] = [
    PronounProfileStr::HE_HIM,
    PronounProfileStr::SHE_HER,
    PronounProfileStr::THEY_THEM,
    PronounProfileStr::PER_PERS,
    PronounProfileStr::IT_ITS,
    PronounProfileStr::FAE_FAER,
    PronounProfileStr::XE_XYRS,
    PronounProfileStr::ZE_ZIE,
    PronounProfileStr::AE_AERS,
];

#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Pronouns {
    HeHim,
    SheHer,
    #[default]
    TheyThem,
    PerPers,
    ItIts,
    FaeFaer,
    XeXyrs,
    ZeZie,
    AeAers,
    AnyAll, // THIS IS RNG!!!
    Custom(PronounProfile),
}

impl Pronouns {
    pub fn decode<S: AsRef<str>>(nouns: S) -> Option<Pronouns> {
        match nouns.as_ref().to_lowercase().as_ref() {
            "he/him" => Some(Pronouns::HeHim),
            "she/her" => Some(Pronouns::SheHer),
            "they/them" => Some(Pronouns::TheyThem),
            "per/pers" => Some(Pronouns::PerPers),
            "it/its" => Some(Pronouns::ItIts),
            "fae/faer" | "fae/faers" => Some(Pronouns::FaeFaer),
            "xe" | "xe/xyrs" | "xe/xers" | "xe/xem" => Some(Pronouns::XeXyrs),
            "ze" | "ze/zie" | "ze/zers" | "ze/zirs" => Some(Pronouns::ZeZie),
            "ae/aers" => Some(Pronouns::AeAers),
            "any/all" => Some(Pronouns::AnyAll),
            _ => None,
            // "he/him" => Some(PronounProfile::new("he", "him", "his", "his", "himself")),
            // "she/her" => Some(PronounProfile::new("she", "her", "hers", "her", "herself")),
            // "they/them" => Some(PronounProfile::new(
            //     "they", "them", "theirs", "their", "themself",
            // )),
            // "per/pers" => Some(PronounProfile::new("per", "per", "per", "pers", "perself")),
            // "it/its" => Some(PronounProfile::new("it", "it", "its", "its", "itself")),
            // "fae/faer" | "fae/faers" => Some(PronounProfile::new(
            //     "fae", "faer", "faer", "faers", "faerself",
            // )),
            // "xe" | "xe/xyrs" | "xe/xers" | "xe/xem" => {
            //     Some(PronounProfile::new("xe", "xem", "xyr", "xyrs", "xemself"))
            // }
            // "ze" | "ze/zie" | "ze/zers" | "ze/zirs" => {
            //     Some(PronounProfile::new("ze", "zir", "zir", "zirs", "zirself"))
            // }
            // "ae/aers" => Some(PronounProfile::new("ae", "aer", "aer", "aers", "aerself")),
            // _ => None,
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
crate::impl_sea_orm!(Pronouns, PronounProfile, PronounProfileStr);
#[cfg(feature = "server")]
crate::impl_redis!(Pronouns, PronounProfile, PronounProfileStr);

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

impl<'a> From<PronounProfile> for PronounProfileStr<'a> {
    fn from(pp: PronounProfile) -> Self {
        PronounProfileStr {
            nominative: Cow::from(&pp.nominative),
            accusative: Cow::from(&pp.accusative),
            pronominal: Cow::from(&pp.pronominal),
            predicative: Cow::from(&pp.predicative),
            reflexive: Cow::from(&pp.reflexive),
        }
    }
}
//
// #[macro_use]
// macro_rules! define_const_ppstr {
//     { $($name:ident : [$nominative:expr, $accusative:expr, $pronominal:expr, $predicative:expr, $reflexive:expr]),* } => {
//         $(
//             pub const $name: PronounProfileStr<'static> = PronounProfileStr {
//                 nominative: Cow::Borrowed($nominative),
//                 accusative: Cow::Borrowed($accusative),
//                 pronominal: Cow::Borrowed($pronominal),
//                 predicative: Cow::Borrowed($predicative),
//                 reflexive: Cow::Borrowed($reflexive),
//             };
//         )*
//     };
// }

#[derive(
    Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize,
)]
pub struct PronounProfileStr<'a> {
    pub nominative: Cow<'a, str>,
    pub accusative: Cow<'a, str>,
    pub pronominal: Cow<'a, str>,
    pub predicative: Cow<'a, str>,
    pub reflexive: Cow<'a, str>,
}

impl<'a> PronounProfileStr<'a> {
    pub const HE_HIM: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("he"),
        accusative: Cow::from("him"),
        pronominal: Cow::from("his"),
        predicative: Cow::from("his"),
        reflexive: Cow::from("himself"),
    };
    pub const SHE_HER: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("she"),
        accusative: Cow::from("her"),
        pronominal: Cow::from("hers"),
        predicative: Cow::from("her"),
        reflexive: Cow::from("herself"),
    };
    pub const THEY_THEM: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("they"),
        accusative: Cow::from("them"),
        pronominal: Cow::from("theirs"),
        predicative: Cow::from("their"),
        reflexive: Cow::from("themself"),
    };
    pub const PER_PERS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("per"),
        accusative: Cow::from("per"),
        pronominal: Cow::from("per"),
        predicative: Cow::from("pers"),
        reflexive: Cow::from("perself"),
    };
    pub const IT_ITS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("it"),
        accusative: Cow::from("it"),
        pronominal: Cow::from("its"),
        predicative: Cow::from("its"),
        reflexive: Cow::from("itself"),
    };
    pub const FAE_FAER: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("fae"),
        accusative: Cow::from("faer"),
        pronominal: Cow::from("faer"),
        predicative: Cow::from("faers"),
        reflexive: Cow::from("faerself"),
    };
    pub const XE_XYRS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("xe"),
        accusative: Cow::from("xem"),
        pronominal: Cow::from("xyr"),
        predicative: Cow::from("xyrs"),
        reflexive: Cow::from("xemself"),
    };
    pub const ZE_ZIE: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("ze"),
        accusative: Cow::from("zir"),
        pronominal: Cow::from("zir"),
        predicative: Cow::from("zirs"),
        reflexive: Cow::from("zirself"),
    };
    pub const AE_AERS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::from("ae"),
        accusative: Cow::from("aer"),
        pronominal: Cow::from("aer"),
        predicative: Cow::from("aers"),
        reflexive: Cow::from("aerself"),
    };
}

impl<'a> From<&'a PronounProfile> for PronounProfileStr<'a> {
    fn from(pp: &'a PronounProfile) -> Self {
        PronounProfileStr {
            nominative: Cow::from(pp.nominative),
            accusative: Cow::from(pp.accusative),
            pronominal: Cow::from(pp.pronominal),
            predicative: Cow::from(pp.predicative),
            reflexive: Cow::from(pp.reflexive),
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
