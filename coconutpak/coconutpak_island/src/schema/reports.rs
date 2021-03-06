use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, EntityTrait, EnumIter, Related, RelationDef,
    RelationTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub report_id: u64,
    pub reporter: u64,
    pub target_pak: u64,
    pub date: DateTime<Utc>,
    #[sea_orm(column_type = "Text")]
    pub reason: String,
    #[sea_orm(column_type = "Text")]
    pub version: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    CoconutPak,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Reporter)
                .to(super::user::Column::Id)
                .into(),
            Relation::CoconutPak => Entity::belongs_to(super::coconutpak::Entity)
                .from(Column::TargetPak)
                .to(super::coconutpak::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::coconutpak::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPak.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
