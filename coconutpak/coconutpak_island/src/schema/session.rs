use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, EntityTrait, EnumIter, Related, RelationDef,
    RelationTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub owner: Uuid,
    #[sea_orm(column_type = "Text")]
    pub ip_address: String,
    #[sea_orm(column_type = "Text")]
    pub hostname: String,
    #[sea_orm(unique, column_type = "Text")]
    pub session_hashed_sha512: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    SessionLog,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Owner)
                .to(super::user::Column::Uuid)
                .into(),
            Relation::SessionLog => Entity::has_many(super::session_log::Entity).into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::session_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SessionLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
