use chrono::{DateTime, Utc};
use kindkapibari_core::dbarray::DBArray;
use kindkapibari_core::preferences::Preferences;
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    sea_query::ValueType,
    ActiveModelBehavior, DerivePrimaryKey, DeriveRelation, EnumIter, IdenStatic, Related,
    RelationDef, TryGetable,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "user_passwords")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u64,
    owner: u64,
    last_changed: DateTime<Utc>,
    #[sea_orm(unique, indexed)]
    pub password_hashed: Vec<u8>,
    #[sea_orm(unique, indexed)]
    pub salt: DBArray<u8, 32>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Owner)
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
