use crate::text::TextContainer;
use semver::{Version, VersionReq};
use staticvec::StaticVec;
use url::Url;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub author: Vec<String>,
    pub name: String,
    pub version: Version,
    pub compatibility: VersionReq,
    pub source: Option<Url>,
    pub description: Option<String>,
    pub tags: Option<StaticVec<String, 5>>,
    pub docs: Option<Url>,
    pub homepage: Option<Url>,
    pub categories: Option<StaticVec<String, 5>>,
    // files
    pub register_text_containers: Option<Vec<TextContainer>>,
}
