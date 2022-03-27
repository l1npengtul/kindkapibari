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
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub handle: String,
    pub gender: Gender,
    pub pronouns: Pronouns,
    pub register_date: DateTime<Utc>,
    #[sea_orm(nullable)]
    pub birthday: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    Preferences,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Preferences => Entity::has_one(super::preferences::Entity).into(),
        }
    }
}

impl Related<super::preferences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Preferences.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for UserData {
    fn from(m: Model) -> Self {
        UserData {
            username: m.username,
            handle: m.handle,
            gender: m.gender,
            pronouns: m.pronouns,
            birthday: m.birthday,
            registered_date: m.register_date,
        }
    }
}