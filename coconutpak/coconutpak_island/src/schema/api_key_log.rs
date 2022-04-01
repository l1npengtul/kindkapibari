use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "api_key_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u64,
    session_id: u64,
    #[sea_orm(column_type = "Text")]
    ip_address: String,
}
