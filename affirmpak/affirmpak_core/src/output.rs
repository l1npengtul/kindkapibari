use crate::text::TextContainer;
use capybafirmations_commons::responses::Response;
use semver::{Version, VersionReq};
use staticvec::StaticVec;
use url::Url;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Output {
    pub author: Vec<String>,
    pub namespace: String,
    pub name: String,
    pub version: Version,
    pub compatibility: VersionReq,
    pub source: Url,
    pub description: String,
    pub tags: StaticVec<String, 5>,
    pub docs: Option<Url>,
    pub homepage: Option<Url>,
    pub catagories: Vec<String>,
    pub readme: Option<Url>,
    // files
    pub register_text_conntainers: Vec<TextContainer>,
}
