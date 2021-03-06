use crate::version::Version;
use serde::{Deserialize, Serialize};
use staticvec::StaticVec;
use url::Url;

pub const CURRENT_MANIFEST_VERSION: Version = Version::new(0, 1, 0);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoconutPakManifest {
    pub author: Vec<String>,
    pub name: String,
    pub edition: Version,
    #[serde(default)]
    pub license: String,
    pub version: Version,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Url>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub readme: String,
    #[serde(default, skip_serializing_if = "StaticVec::is_empty")]
    pub tags: StaticVec<String, 5>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Url>,
    #[serde(default, skip_serializing_if = "StaticVec::is_empty")]
    pub categories: StaticVec<String, 5>,
}

impl Default for CoconutPakManifest {
    fn default() -> Self {
        CoconutPakManifest {
            author: vec![],
            name: "".to_string(),
            edition: CURRENT_MANIFEST_VERSION,
            license: "".to_string(),
            version: Version::new(0, 1, 0),
            source: None,
            description: "".to_string(),
            readme: "".to_string(),
            tags: StaticVec::default(),
            docs: None,
            homepage: None,
            categories: StaticVec::default(),
        }
    }
}
