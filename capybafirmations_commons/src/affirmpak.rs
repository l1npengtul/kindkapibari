use crate::languages::Languages;
use crate::tags::Tags;
#[cfg(feature = "server")]
use sea_orm::*;
use semver::Version;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(DeriveEntityModel))]
#[cfg_attr(feature = "server", sea_orm(table_name = "affirm_packs"))]
pub struct Model {
    #[cfg_attr(feature = "server", sea_orm(primary_key))]
    pub id: Uuid,
    pub name: String,
    pub author: Uuid,
    pub version: Version,
    #[cfg_attr(feature = "server", sea_orm(column_type = "JsonBinary"))]
    pub language: Languages,
    #[cfg_attr(feature = "server", sea_orm(column_type = "JsonBinary"))]
    pub tags: Vec<Tags>,
    pub downloads: u64,
    pub likes: u64,
    pub source: String,
    pub data: String,
}

#[cfg(feature = "server")]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "server", derive(EnumIter))]
pub enum Relation {}

#[cfg(feature = "server")]
impl ActiveModelBehavior for ActiveModel {}
