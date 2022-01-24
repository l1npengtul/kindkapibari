use language_tag::LanguageTag;

pub enum PakType {
    Theme,
    Phrases,
    Extension,
}

pub struct AffirmPak {
    pak_type: PakType,
    language: Option<Vec<LanguageTag>>,
    tags: Vec<String>,
}