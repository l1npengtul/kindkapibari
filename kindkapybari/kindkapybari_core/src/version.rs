use semver::Version;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SemVer {
    version: Version,
}

impl Deref for SemVer {
    type Target = Version;

    fn deref(&self) -> &Self::Target {
        &self.version
    }
}

impl DerefMut for SemVer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.version
    }
}
