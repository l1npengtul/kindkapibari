use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "coconutpaks")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: Uuid,
    #[sea_orm(unique, indexed)]
    name: String,
    size: u64,
    downloads: u64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CoconutPakHistory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::CoconutPakHistory => {
                Entity::has_many(super::coconutpak_history::Entity).into()
            }
        }
    }
}

impl Related<super::coconutpak_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPakHistory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
