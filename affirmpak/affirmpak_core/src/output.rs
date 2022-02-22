use crate::text::TextContainer;
use capybafirmations_commons::responses::Response;
use semver::{Version, VersionReq};
use staticvec::StaticVec;
use url::Url;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    pub catagories: Option<StaticVec<String, 5>>,
    // files
    pub register_text_conntainers: Option<Vec<TextContainer>>,
}
