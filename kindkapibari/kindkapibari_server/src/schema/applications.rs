use chrono::{DateTime, Utc};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "application")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub creator: u64,
    pub name: String,
    pub domain: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::users::user::Entity)
                .from(Column::Creator)
                .to(super::users::user::Column::Id)
                .into(),
        }
    }
}

impl Related<super::users::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
