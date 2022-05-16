use chrono::{Date, DateTime, Utc};
use kindkapibari_core::{gender::Gender, pronouns::Pronouns};
use language_tags::LanguageTag;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use kindkapibari_core::dbarray::DBArray;

// TODO: support encrypted Userdata to protect our users

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "user_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: u64,
    #[sea_orm(nullable)]
    pub nonce: Option<DBArray<u8, 24>>,
    pub encrypted: bool,
    pub user_data: Vec<u8>,
}

impl Model {
    pub fn decrypt()
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
