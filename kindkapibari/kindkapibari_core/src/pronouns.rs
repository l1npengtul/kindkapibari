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
    pub nominative: Cow<'a, str>,
    pub accusative: Cow<'a, str>,
    pub pronominal: Cow<'a, str>,
    pub predicative: Cow<'a, str>,
    pub reflexive: Cow<'a, str>,
}

impl<'a> PronounProfileStr<'a> {
    pub const HE_HIM: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("he"),
        accusative: Cow::Borrowed("him"),
        pronominal: Cow::Borrowed("his"),
        predicative: Cow::Borrowed("his"),
        reflexive: Cow::Borrowed("himself"),
    };
    pub const SHE_HER: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("she"),
        accusative: Cow::Borrowed("her"),
        pronominal: Cow::Borrowed("hers"),
        predicative: Cow::Borrowed("her"),
        reflexive: Cow::Borrowed("herself"),
    };
    pub const THEY_THEM: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("they"),
        accusative: Cow::Borrowed("them"),
        pronominal: Cow::Borrowed("theirs"),
        predicative: Cow::Borrowed("their"),
        reflexive: Cow::Borrowed("themself"),
    };
    pub const PER_PERS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("per"),
        accusative: Cow::Borrowed("per"),
        pronominal: Cow::Borrowed("per"),
        predicative: Cow::Borrowed("pers"),
        reflexive: Cow::Borrowed("perself"),
    };
    pub const IT_ITS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("it"),
        accusative: Cow::Borrowed("it"),
        pronominal: Cow::Borrowed("its"),
        predicative: Cow::Borrowed("its"),
        reflexive: Cow::Borrowed("itself"),
    };
    pub const FAE_FAER: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("fae"),
        accusative: Cow::Borrowed("faer"),
        pronominal: Cow::Borrowed("faer"),
        predicative: Cow::Borrowed("faers"),
        reflexive: Cow::Borrowed("faerself"),
    };
    pub const XE_XYRS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("xe"),
        accusative: Cow::Borrowed("xem"),
        pronominal: Cow::Borrowed("xyr"),
        predicative: Cow::Borrowed("xyrs"),
        reflexive: Cow::Borrowed("xemself"),
    };
    pub const ZE_ZIE: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("ze"),
        accusative: Cow::Borrowed("zir"),
        pronominal: Cow::Borrowed("zir"),
        predicative: Cow::Borrowed("zirs"),
        reflexive: Cow::Borrowed("zirself"),
    };
    pub const AE_AERS: PronounProfileStr<'static> = PronounProfileStr {
        nominative: Cow::Borrowed("ae"),
        accusative: Cow::Borrowed("aer"),
        pronominal: Cow::Borrowed("aer"),
        predicative: Cow::Borrowed("aers"),
        reflexive: Cow::Borrowed("aerself"),
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

#[cfg(feature = "server")]
crate::impl_sea_orm!(Pronouns, PronounProfile, PronounProfileStr<'_>);
#[cfg(feature = "server")]
crate::impl_redis!(Pronouns, PronounProfile, PronounProfileStr<'_>);
