use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    sea_query::ValueType,
    ActiveModelBehavior, EnumIter, IdenStatic, Related, RelationDef, TryGetable,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "sobers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub owner: u64,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub time_since_reset: DateTime<Utc>,
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
