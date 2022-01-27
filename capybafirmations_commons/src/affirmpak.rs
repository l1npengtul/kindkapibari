use crate::languages::Languages;
use crate::tags::Tags;
use crate::user::{User, UserID};
use chrono::{DateTime, Utc};
use semver::Version;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum PakType {
    Theme,
    Phrases,
    Extension,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AffirmPak {
    id: u64,
    author: UserID,
    date: DateTime<Utc>,
    version: Version,
    pak_type: PakType,
    language: Option<Vec<Languages>>,
    tags: Vec<Tags>,
    data: String,
}
