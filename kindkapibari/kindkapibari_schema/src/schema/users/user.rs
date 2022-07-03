use chrono::{DateTime, Utc};
use kindkapibari_core::{impl_redis, roles::Role};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    ActiveModelBehavior, DerivePrimaryKey, EnumIter, IdenStatic, Related, RelationDef,
};
use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(
    Clone,
    Debug,
    Hash,
    PartialOrd,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    DeriveEntityModel,
    Component,
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
    #[sea_orm(column_type = "JsonBinary")]
    pub roles: Role,
}

#[derive(Copy, Clone, Debug, EnumIter)]
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
    Statistics,
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
            Relation::Statistics => Entity::has_one(super::statistics::Entity).into(),
        }
    }
}

impl Related<super::super::applications::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Applications.def()
    }
}

impl Related<super::oauth_authorizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Authorizations.def()
    }
}

impl Related<super::badges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Badges.def()
    }
}

impl Related<super::super::bans::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bans.def()
    }
}

impl Related<super::connections::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Connections.def()
    }
}

impl Related<super::login_tokens::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoginTokens.def()
    }
}

impl Related<super::passwords::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Passwords.def()
    }
}

impl Related<super::preferences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Preferences.def()
    }
}

impl Related<super::userdata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserData.def()
    }
}

impl Related<super::onetime_reminders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OneTimeReminders.def()
    }
}

impl Related<super::recurring_reminders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RecurringReminders.def()
    }
}

impl Related<super::sobers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sobers.def()
    }
}

impl Related<super::statistics::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Statistics.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl_redis!(Model);
