use std::ops::Deref;

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
    Custon(PronounProfile),
}

impl Pronouns {
    pub fn decode<S: AsRef<str>>(nouns: S) -> Option<Pronouns> {
        match nouns.as_ref().to_lowercase().as_ref() {
            "he/him" => {}
            "she/her" => {}
            "they/them" => {}
            _ => None,
        }
    }
}

#[derive(Clone, Default, Hash, PartialOrd, PartialEq)]
pub struct PronounProfile {
    nominative: String,
    accusative: String,
    pronominal: String,
    predicative: String,
    reflexive: String,
}

impl PronounProfile {
    pub fn new(subject: String, object: String, possessive: String) -> Self {
        PronounProfile {
            nominative: subject,
            accusative: object,
            pron,
        }
    }

    pub fn subject(&self) -> &str {
        &self.nominative
    }
    pub fn object(&self) -> &str {
        &self.accusative
    }
    pub fn possessive(&self) -> &str {
        &self.pron
    }

    pub fn set_subject(&mut self, subject: String) {
        self.nominative = subject;
    }
    pub fn set_object(&mut self, object: String) {
        self.accusative = object;
    }
    pub fn set_possessive(&mut self, possessive: String) {
        self.pron = possessive;
    }
}
