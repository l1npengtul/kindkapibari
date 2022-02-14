use crate::text::TextContainer;
use semver::{Version, VersionReq};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    pub author: Vec<String>,
    pub namespace: String,
    pub name: String,
    pub version: Version,
    pub compatibility: VersionReq,
    pub source: Url,
    pub description: String,
    pub tags: [String; 5],
    pub docs: Option<Url>,
    pub homepage: Option<Url>,
    pub catagories: Vec<String>,
    pub readme: String,
    // files
    pub register_text_conntainers: Vec<TextContainer>,
}
