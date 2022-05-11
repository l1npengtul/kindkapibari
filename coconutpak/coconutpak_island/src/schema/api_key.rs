use chrono::DateTime;
use chrono::Utc;
use kindkapibari_core::dbarray::DBArray;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, EntityTrait, EnumIter, Related, RelationDef,
    RelationTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "api_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub owner: u64,
    pub created: DateTime<Utc>,
    #[sea_orm(unique, indexed)]
    pub key_hashed: Vec<u8>,
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
            Relation::User => Entity::belongs_to(super::user::Entity)
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
