use sea_orm::{ActiveEnum, ColumnDef, ColumnType, DbErr, EnumIter};

// UNUSED! To be finalized in a future COCONUTPAK version.

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(EnumIter))]
pub enum GenderStyleMarker {
    Masc,
    Fem,
    Andro,
    Other(String),
}

#[cfg(feature = "server")]
impl ActiveEnum for GenderStyleMarker {
    type Value = String;

    fn name() -> String {
        "gender_style_market".to_string()
    }

    fn to_value(&self) -> Self::Value {
        match self {
            GenderStyleMarker::Masc => "MASC",
            GenderStyleMarker::Fem => "FEM",
            GenderStyleMarker::Andro => "ANDRO",
            GenderStyleMarker::Other(v) => v.as_str(),
        }
        .to_owned()
    }

    fn try_from_value(v: &Self::Value) -> Result<Self, DbErr> {
        Ok(match v.as_str() {
            "MASC" => GenderStyleMarker::Masc,
            "FEM" => GenderStyleMarker::Fem,
            "ANDRO" => GenderStyleMarker::Andro,
            gsm => GenderStyleMarker::Other(gsm.to_string()),
        })
    }

    fn db_type() -> ColumnDef {
        ColumnType::String(Some(1)).def()
    }
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(EnumIter))]
pub enum Tags {
    Fem,
    TransFem,
    CisFem,
    Masc,
    TransMasc,
    CisMasc,
    Andro,
    NonBinary(String),
    VoiceTraining(GenderStyleMarker),
    Dysphoria(GenderStyleMarker),
    Profession(String),
    Transition(GenderStyleMarker),
    Presentation(GenderStyleMarker),
    AntiTerf,
    AntiChud,
    Other(String),
}

#[cfg(feature = "server")]
impl ActiveEnum for Tags {
    type Value = String;

    fn name() -> String {
        "tags".to_string()
    }

    fn to_value(&self) -> Self::Value {
        match self {
            Tags::Fem => "FEM",
            Tags::TransFem => "TRANSFEM",
            Tags::CisFem => "CISFEM",
            Tags::Masc => "MASC",
            Tags::TransMasc => "TRANSMASC",
            Tags::CisMasc => "CISMASC",
            Tags::Andro => "ANDRO",
            Tags::NonBinary(nb) => "NONBINARY_" + nb.as_str(),
            Tags::VoiceTraining(vt) => "VOICETRAINING_" + vt.to_value().as_str(),
            Tags::Dysphoria(d) => "DYSPHORIA_" + d.to_value().as_str(),
            Tags::Profession(w) => "PROFESSION_" + w.as_str(),
            Tags::Transition(tr) => "TRANSITION_" + tr.to_value().as_str(),
            Tags::Presentation(pe) => "PRESENTATION_" + pe.to_value().as_str(),
            Tags::AntiTerf => "ANTITERF",
            Tags::AntiChud => "ANTICHUD",
            Tags::Other(o) => o.as_str(),
        }
        .to_owned()
    }

    fn try_from_value(v: &Self::Value) -> Result<Self, DbErr> {
        Ok(match v.as_str() {
            o => Tags::Other(o.to_string()),
        })
    }

    fn db_type() -> ColumnDef {
        todo!()
    }
}
