use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, EntityTrait, EnumIter, Related, RelationDef,
    RelationTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "session_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u64,
    session_id: u64,
    #[sea_orm(column_type = "Text")]
    hostname: String,
    #[sea_orm(column_type = "Text")]
    ip_address: String,
    when: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Session,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Session => Entity::belongs_to(super::session::Entity)
                .from(Column::SessionId)
                .to(super::session::Column::Id)
                .into(),
        }
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
