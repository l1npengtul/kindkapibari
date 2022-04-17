use crate::scopes::Scopes;
use chrono::{DateTime, Utc};
use kindkapibari_core::dbvec::DBVec;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "oauth")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub user: u64,
    #[sea_orm(column_type = "Text", indexed, unique)]
    pub access: String,
    #[sea_orm(column_type = "Text", nullable, indexed, unique)]
    pub refresh: Option<String>,
    pub expire: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub scopes: DBVec<Scopes>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::users::user::Entity)
                .from(Column::User)
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
