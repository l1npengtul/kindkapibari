use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "application_secrets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub app_id: u64,
    #[sea_orm(column_type = "Text", indexed)]
    pub six_char_short: String,
    #[sea_orm(column_type = "Text", indexed, unique)]
    pub secret_hash: String,
    pub created: DateTime<Utc>,
    pub active: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Applications,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Applications => Entity::belongs_to(super::applications::Entity)
                .from(Column::AppId)
                .to(super::applications::Column::Id)
                .into(),
        }
    }
}

impl Related<super::applications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Applications.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
