#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Transition {
    Masc,
    Fem,
    Andro,
    Other(String),
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum Tags {
    Fem,
    TransFem,
    CisFem,
    Masc,
    TransMasc,
    CisMasc,
    NonBinary,
    XenoGender(String),
    VoiceTraining,
    Dysphoria,
    Profession(String),
    Transition(Transition),
    Appearance(Transition),
    AntiTerf,
    AntiChud,
    Other(String),
}
