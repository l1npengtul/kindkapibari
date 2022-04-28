use bson::spec::BinarySubtype::Column;
use chrono::{DateTime, Utc};
use kindkapibari_core::{
    dbarray::DBArray, dbvec::DBVec, manifest::CoconutPakManifest, version::Version,
};
use poem_openapi::{
    registry::{MetaSchema, MetaSchemaRef},
    types::{ToJSON, Type},
};
use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, EnumIter, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "oauth2_kkb_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub token_id: u64,
    pub owner: u64,
    #[sea_orm(column_type = "Text")]
    pub authorization_token: String,
    #[sea_orm(column_type = "Text")]
    pub refresh_token: String,
    pub expire: DateTime<Utc>,
    pub scopes: DBVec<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Owner)
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
