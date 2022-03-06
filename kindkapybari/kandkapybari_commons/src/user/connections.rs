#[cfg(feature = "server")]
use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(DeriveEntityModel))]
#[cfg_attr(feature = "server", sea_orm(table_name = "user_connections"))]
pub struct Model {
    #[cfg_attr(feature = "server", sea_orm(primary_key))]
    pub id: Uuid,
    pub twitter: String,
    pub github: Option<String>,
}

#[cfg(feature = "server")]
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

#[cfg(feature = "server")]
impl ActiveModelBehavior for ActiveModel {}
