#[cfg(feature = "server")]
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(DeriveEntityModel))]
#[cfg_attr(feature = "server", sea_orm(table_name = "users"))]
pub struct Model {
    #[cfg_attr(feature = "server", sea_orm(primary_key))]
    pub id: Uuid,
    pub username: String,
    pub handle: String,
}

#[cfg(feature = "server")]
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    #[cfg_attr(feature = "server", sea_orm(has_many = "super::connections::Entity"))]
    Connections,
    #[cfg_attr(feature = "server", sea_orm(has_many = "super::metadata::Entity"))]
    Metadata,
    #[cfg_attr(feature = "server", sea_orm(has_many = "super::settings::Entity"))]
    Settings,
}

#[cfg(feature = "server")]
impl ActiveModelBehavior for ActiveModel {}
