use poem_openapi::registry::Registry;
use poem_openapi::Tags;
use serde::{Deserialize, Serialize};

pub mod v1;

#[derive(
    Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Tags,
)]
pub enum VersionTags {
    V1,
}
