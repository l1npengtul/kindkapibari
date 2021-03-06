use oauth2::url::Url;
use poem_openapi::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{ToJSON, Type},
};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    strum::IntoEnumIterator,
    ActiveModelBehavior, EnumIter, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "subscribers")]
pub struct Model {
    #[sea_orm(primary_key)]
    user_id: u64,
    #[sea_orm(primary_key)]
    pak_id: u64,
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
                .from(Column::UserId)
                .to(super::user::Column::Id)
                .into(),
            Relation::CoconutPak => Entity::belongs_to(super::coconutpak::Entity)
                .from(Column::PakId)
                .to(super::coconutpak::Column::Id)
                .into(),
        }
    }
}
