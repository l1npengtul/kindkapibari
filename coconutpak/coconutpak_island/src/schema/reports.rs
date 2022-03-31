use sea_orm::DeriveEntityModel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub reporter: Uuid,
    pub target: Uuid,
    pub coconutpak: Uuid,
    #[sea_orm(column_type = "Text")]
    pub reason: String,
}
