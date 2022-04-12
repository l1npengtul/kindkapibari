use chrono::{DateTime, Utc};
use kindkapybari_core::{gender::Gender, pronouns::Pronouns, user_data::UserData};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    sea_query::ValueType,
    ActiveModelBehavior, DerivePrimaryKey, DeriveRelation, EnumIter, IdenStatic, Related,
    RelationDef, TryGetable,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "user_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
}
