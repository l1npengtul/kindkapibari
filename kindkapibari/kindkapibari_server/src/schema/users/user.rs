use crate::roles::Roles;
use chrono::{DateTime, Utc};
use kindkapibari_core::dbvec::DBVec;
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
    pub id: u64,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text", unique, indexed)]
    pub handle: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub email: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub profile_pictures: Option<String>,
    pub roles: DBVec<Roles>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    Preferences,
    Connections,
    UserData,
    Bans,
    Applications,
    Authorizations,
    LoginTokens,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Preferences => Entity::has_one(super::preferences::Entity).into(),
            Relation::Connections => Entity::has_one(super::connections::Entity).into(),
            Relation::UserData => Entity::has_one(super::userdata::Entity).into(),
            Relation::Bans => Entity::has_many(super::super::bans::Entity).into(),
            Relation::Applications => Entity::has_many(super::super::applications::Entity).into(),
            Relation::Authorizations => {
                Entity::has_many(super::oauth_authorizations::Entity).into()
            }
            Relation::LoginTokens => Entity::has_many(super::login_tokens::Entity).into(),
        }
    }
}

impl Related<super::preferences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Preferences.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
