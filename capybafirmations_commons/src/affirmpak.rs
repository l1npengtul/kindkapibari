use crate::languages::Languages;
use crate::tags::Tags;
use semver::Version;
use url::Url;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(DeriveEntityModel))]
#[cfg_attr(feature = "server", sea_orm(table_name = "affirm_pak"))]
pub struct AffirmPak {
    #[cfg_attr(feature = "server", sea_orm(primary_key))]
    pub id: Uuid,
    pub name: String,
    pub author: Uuid,
    pub version: Version,
    pub language: Option<Languages>,
    pub tags: Vec<Tags>,
    pub downloads: u64,
    pub likes: u64,
    pub source: Url,
    pub data: Url,
}
