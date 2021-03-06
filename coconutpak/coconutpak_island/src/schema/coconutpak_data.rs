use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, EnumIter, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "coconutpak_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    #[sea_orm(column_type = "Text")]
    pub archive_link: String,
    #[sea_orm(column_type = "Text")]
    pub compiled_link: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CoconutPakHistory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::CoconutPakHistory => Entity::belongs_to(super::coconutpak_versions::Entity)
                .from(Column::Id)
                .to(super::coconutpak_versions::Column::Id)
                .into(),
        }
    }
}

impl Related<super::coconutpak_versions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPakHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
