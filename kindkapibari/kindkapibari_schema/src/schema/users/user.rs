use chrono::{DateTime, Utc};
use kindkapibari_core::roles::Roles;
use kindkapibari_core::{dbvec::DBVec, impl_redis};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    sea_query::ValueType,
    ActiveModelBehavior, DerivePrimaryKey, DeriveRelation, EnumIter, IdenStatic, Related,
    RelationDef, TryGetable,
};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    #[sea_orm(column_type = "Text", unique, indexed)]
    pub username: String,
    #[sea_orm(column_type = "Text", unique, indexed)]
    pub email: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub profile_picture: Option<String>,
    pub creation_date: DateTime<Utc>,
    pub roles: DBVec<Roles>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    Applications,
    Authorizations,
    Badges,
    Bans,
    Connections,
    LoginTokens,
    Passwords,
    Preferences,
    UserData,
    OneTimeReminders,
    RecurringReminders,
    Sobers,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Applications => Entity::has_many(super::super::applications::Entity).into(),
            Relation::Authorizations => {
                Entity::has_many(super::oauth_authorizations::Entity).into()
            }
            Relation::Badges => Entity::has_one(super::badges::Entity).into(),
            Relation::Bans => Entity::has_many(super::super::bans::Entity).into(),
            Relation::Connections => Entity::has_one(super::connections::Entity).into(),
            Relation::LoginTokens => Entity::has_many(super::login_tokens::Entity).into(),
            Relation::Passwords => Entity::has_one(super::passwords::Entity).into(),
            Relation::Preferences => Entity::has_one(super::preferences::Entity).into(),
            Relation::UserData => Entity::has_one(super::userdata::Entity).into(),
            Relation::OneTimeReminders => Entity::has_many(super::onetime_reminders::Entity).into(),
            Relation::RecurringReminders => {
                Entity::has_many(super::recurring_reminders::Entity).into()
            }
            Relation::Sobers => Entity::has_many(super::sobers::Entity).into(),
        }
    }
}

impl Related<super::preferences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Preferences.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl_redis!(Model);
