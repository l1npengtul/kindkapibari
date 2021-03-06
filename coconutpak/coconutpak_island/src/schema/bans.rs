use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "bans")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user: u64,
    #[sea_orm(column_type = "Timestamp", nullable)]
    pub until: Option<DateTime<Utc>>,
    #[sea_orm(column_type = "Text", nullable)]
    pub reason: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub ip: Option<String>, // if not none is IPBAN
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::user::Entity)
                .from(Column::User)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
