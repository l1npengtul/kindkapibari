use kindkapibari_core::secret::RefreshClaims;
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, RelationTrait},
    ActiveModelBehavior, DerivePrimaryKey, EnumIter, IdenStatic, Related, RelationDef,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "refresh_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub owner: u64,
    pub related: u64,
    pub expire: u64,
    pub created: u64,
    pub revoked: bool,
    #[sea_orm(column_type = "JsonBinary", unique, indexed)]
    pub stored_secret: RefreshClaims,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    // LoginTokens,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
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

impl ActiveModelBehavior for ActiveModel {}
