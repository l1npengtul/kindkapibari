use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    sea_query::ValueType,
    ActiveModelBehavior, EnumIter, IdenStatic, Related, RelationDef, TryGetable,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "statistics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub owner: u64,
    pub hours_played: u64,
}
