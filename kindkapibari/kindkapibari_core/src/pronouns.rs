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

#[cfg_attr(feature = "server", derive(utoipa::Component))]
#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
#[non_exhaustive]
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

impl Default for Pronouns {
    fn default() -> Self {
        Pronouns::TheyThem
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

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::Component))]
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

    #[must_use]
    pub fn verify(&self) -> bool {
        !(self.reflexive.len() > 30
            || self.accusative.len() > 30
            || self.pronominal.len() > 30
            || self.predicative.len() > 30
            || self.reflexive.len() > 30)
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

impl Default for PronounProfile {
    fn default() -> Self {
        // FIXME: check this mayb???
        Self {
            nominative: "they".to_string(),
            accusative: "they".to_string(),
            pronominal: "their".to_string(),
            predicative: "their".to_string(),
            reflexive: "themself".to_string(),
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

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct PronounProfileStr<'a> {
    pub nominative: &'a str,
    pub accusative: &'a str,
    pub pronominal: &'a str,
    pub predicative: &'a str,
    pub reflexive: &'a str,
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
    fn from(pp: &'a PronounProfile) -> PronounProfileStr<'a> {
        PronounProfileStr {
            nominative: &pp.nominative,
            accusative: &pp.accusative,
            pronominal: &pp.pronominal,
            predicative: &pp.predicative,
            reflexive: &pp.reflexive,
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

#[cfg(feature = "server")]
crate::impl_sea_orm!(Pronouns, PronounProfile);
#[cfg(feature = "server")]
crate::impl_redis!(Pronouns, PronounProfile);
