use kindkapibari_core::badges::Badges;
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    ActiveModelBehavior, DerivePrimaryKey, EnumIter, IdenStatic, Related, RelationDef,
};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "user_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: u64,
    #[sea_orm(column_type = "JsonBinary")]
    pub badges: Badges,
    pub primary: u64,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::UserId)
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
