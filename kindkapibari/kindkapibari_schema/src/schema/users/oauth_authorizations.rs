use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    ActiveModelBehavior, DerivePrimaryKey, EnumIter, IdenStatic, Related, RelationDef,
};
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel,
)]
#[sea_orm(table_name = "oauth_authorizations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub owner: u64,
    pub application: u64,
    pub expire: DateTime<Utc>,
    pub created: DateTime<Utc>,
    // #[sea_orm(unique, indexed)]
    // pub access_token: StoredSecret,
    // #[sea_orm(unique, indexed, nullable)]
    // pub refresh_token: Option<StoredSecret>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Application,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Owner)
                .to(super::user::Column::Id)
                .into(),
            Relation::Application => Entity::belongs_to(super::super::applications::Entity)
                .from(Column::Application)
                .to(super::super::applications::Column::Id)
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
