use bson::serde_helpers::serialize_uuid_as_binary;
use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs, Value};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, EnumIter, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub enum UserPermissions {
    All,
    Admin,
    Demote,
    Promote,
    YankSelf,
    YankOthers,
    Publish,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: u64,
    // pub kindkapibari_id: Uuid,
    pub github_id: u64, // This is for now. TODO: change it back!!!!
    #[sea_orm(column_type = "Text")]
    pub username: String,
    pub restricted_account: bool,
    pub administrator_account: bool,
    pub fake_account: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub email: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ApiKey,
    Session,
    Reports,
    CoconutPak,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::ApiKey => Entity::has_many(super::api_key::Entity).into(),
            Relation::Session => Entity::has_many(super::session::Entity).into(),
            Relation::Reports => Entity::has_many(super::reports::Entity).into(),
            Relation::CoconutPak => Entity::has_many(super::user::Entity).into(),
        }
    }
}

impl Related<super::api_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKey.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::reports::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reports.def()
    }
}

impl Related<super::coconutpak::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPak.def()
    }
}

impl ToRedisArgs for Model {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(&pot::to_vec(&self).unwrap_or_default())
    }
}

impl FromRedisValue for Model {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        return if let Value::Data(bytes) = v {
            pot::from_slice::<Self>(bytes).map_err(|x| RedisError::from(x))
        } else {
            RedisResult::Err(RedisError::from(
                ErrorKind::TypeError,
                "Expected Byte Value",
            ))
        };
    }
}

impl ActiveModelBehavior for ActiveModel {}
