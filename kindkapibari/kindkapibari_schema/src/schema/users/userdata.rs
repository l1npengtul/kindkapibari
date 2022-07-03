use chrono::{DateTime, Utc};
use kindkapibari_core::{
    gender::Gender,
    pronouns::Pronouns,
    user_data::{Locale, UserData},
};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    ActiveModelBehavior, DerivePrimaryKey, EnumIter, IdenStatic, Related, RelationDef,
};
use serde::{Deserialize, Serialize};

// TODO: support encrypted Userdata to protect our users

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "user_data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: u64,
    #[sea_orm(column_type = "JsonBinary")]
    pub gender: Gender,
    #[sea_orm(column_type = "JsonBinary")]
    pub pronouns: Pronouns,
    #[sea_orm(column_type = "DateTime", nullable)]
    pub birthday: Option<DateTime<Utc>>,
    #[sea_orm(column_type = "JsonBinary")]
    pub locale: Locale,
}

impl Model {
    #[must_use]
    pub fn into_userdata(self) -> UserData {
        UserData::new(self.gender, self.pronouns, self.birthday, self.locale)
    }
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
