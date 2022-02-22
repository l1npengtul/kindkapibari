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

#[derive(Clone, Debug, Default, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct PronounProfile {
    nominative: String,
    accusative: String,
    pronominal: String,
    predicative: String,
    reflexive: String,
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

    #[must_use] pub fn nominative(&self) -> &str {
        &self.nominative
    }
    #[must_use] pub fn accusative(&self) -> &str {
        &self.accusative
    }
    #[must_use] pub fn pronominal(&self) -> &str {
        &self.pronominal
    }
    #[must_use] pub fn predicative(&self) -> &str {
        &self.predicative
    }
    #[must_use] pub fn reflexive(&self) -> &str {
        &self.reflexive
    }
}
