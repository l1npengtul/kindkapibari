use chrono::{DateTime, Utc};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "coconutpak_histories")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    coconut_pak: Uuid,
    #[sea_orm(column_type = "Text")]
    version_string: String,
    manifest_bson_lz4: Vec<u8>,
    package_bson_snappy: Vec<u8>,
    release: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CoconutPak,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::CoconutPak => Entity::belongs_to(super::coconutpak::Entity)
                .from(Column::CoconutPak)
                .to(super::coconutpak::Column::Id)
                .into(),
        }
    }
}

impl Related<super::coconutpak::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPak.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
