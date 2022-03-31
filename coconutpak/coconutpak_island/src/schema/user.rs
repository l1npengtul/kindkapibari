use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, EnumIter, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: Uuid,
    pub kindkapybari_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ApiKey,
    Session,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::ApiKey => Entity::has_many(super::api_key::Entity).into(),
            Relation::Session => Entity::has_many(super::session::Entity).into(),
        }
    }
}

impl Related<super::api_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKey.def()
    }
}

impl Related<super::coconutpak::Entity> for Entity {
    fn to() -> RelationDef {
        super::subscribers::Relation::CoconutPak.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::subscribers::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
