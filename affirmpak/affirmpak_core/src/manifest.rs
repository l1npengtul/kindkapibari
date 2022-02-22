use semver::{Version, VersionReq};
use staticvec::StaticVec;
use url::Url;

#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct AffirmPakManifest {
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
}
