use semver::{Version, VersionReq};
use url::Url;

#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct AffirmPakManifest {
    author: Vec<String>,
    namespace: String,
    name: String,
    version: Version,
    compatibility: VersionReq,
    source: Url,
    description: String,
    tags: [String; 5],
    docs: Option<Url>,
    homepage: Option<Url>,
    catagories: Vec<String>,
    readme: String,
}

impl AffirmPakManifest {
    pub fn author(&self) -> &Vec<String> {
        &self.author
    }
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn compatibility(&self) -> &VersionReq {
        &self.compatibility
    }
    pub fn source(&self) -> &Url {
        &self.source
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn tags(&self) -> &[String; 5] {
        &self.tags
    }
    pub fn docs(&self) -> &Option<Url> {
        &self.docs
    }
    pub fn homepage(&self) -> &Option<Url> {
        &self.homepage
    }
    pub fn catagories(&self) -> &Vec<String> {
        &self.catagories
    }
    pub fn readme(&self) -> &str {
        &self.readme
    }
}
