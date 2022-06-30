use crate::KKBScope;
use kindkapibari_core::dbvec::DBVec;
use kindkapibari_core::scopes::KKBScope;
use kindkapibari_core::secret::StoredSecret;
use sea_orm::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "applications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub creator: u64,
    #[sea_orm(column_type = "Text", indexed)]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub homepage: String,
    pub callback: String,
    pub logo: String,
    #[sea_orm(column_type = "Text", indexed, nullable)]
    pub signed_secret: Option<StoredSecret>,
    pub scopes: DBVec<KKBScope>,
    pub confidential: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    ApplicationSecret,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::users::user::Entity)
                .from(Column::Creator)
                .to(super::users::user::Column::Id)
                .into(),
            Relation::ApplicationSecret => {
                Entity::has_many(super::application_secrets::Entity).into()
            }
        }
    }
}

impl Related<super::users::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::application_secrets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApplicationSecret.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
