use crate::preferences::Preferences;
#[cfg(feature = "server")]
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(DeriveEntityModel))]
#[cfg_attr(feature = "server", sea_orm(table_name = "user_settings"))]
pub struct Model {
    #[cfg_attr(feature = "server", sea_orm(primary_key))]
    pub id: Uuid,
    #[cfg_attr(feature = "server", sea_orm(column_type = "JsonBinary"))]
    pub preferences: Preferences,
    #[cfg_attr(feature = "server", sea_orm(column_type = "JsonBinary"))]
    pub paks: Vec<Uuid>,
}

#[cfg(feature = "server")]
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[sea_orm(has_many = "super::affirmpak::Entity")]
    AffirmPak,
}

#[cfg(feature = "server")]
impl ActiveModelBehavior for ActiveModel {}
