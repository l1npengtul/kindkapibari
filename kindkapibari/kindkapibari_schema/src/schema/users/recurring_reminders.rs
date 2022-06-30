use chrono::NaiveTime;
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    sea_query::ValueType,
    ActiveModelBehavior, EnumIter, IdenStatic, RelationDef, TryGetable,
};
use serde::{Deserialize, Serialize};

// TODO: Add encryption support.

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "recurring_reminders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub owner: u64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub days: u8, // use a u8 bitflag, see u8_bitflag_to_days
    pub time: NaiveTime,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, EnumIter)]
pub enum Relations {
    User,
}

impl RelationTrait for Relations {
    fn def(&self) -> RelationDef {
        match self {
            Relations::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Owner)
                .to(Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
