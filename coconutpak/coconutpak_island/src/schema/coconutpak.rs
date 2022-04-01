use chrono::{DateTime, Utc};
use color_eyre::owo_colors::OwoColorize;
use poem_openapi::registry::{MetaSchema, MetaSchemaRef};
use poem_openapi::types::{ToJSON, Type};
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, EnumIter, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "coconutpaks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique, indexed)]
    pub name: String,
    pub subscribers: u32,
    pub last_published: DateTime<Utc>,
    pub verified: bool,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CoconutPakHistory,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::CoconutPakHistory => {
                Entity::has_many(super::coconutpak_history::Entity).into()
            }
        }
    }
}

impl Related<super::coconutpak_history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPakHistory.def()
    }
}

impl Related<super::reports::Entity> for Entity {
    fn to() -> RelationDef {
        super::reports::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::reports::Relation::CoconutPak.def().rev())
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::subscribers::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::subscribers::Relation::CoconutPak.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Type for Model {
    const IS_REQUIRED: bool = false;
    type RawValueType = Self;
    type RawElementValueType = Self;

    fn name() -> Cow<'static, str> {
        Cow::Borrowed("CoconutPak")
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema::new("string")))
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(Self)
    }

    fn raw_element_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a Self::RawElementValueType> + 'a> {
        Box::new(self.as_raw_value().into_iter())
    }
}

impl ToJSON for Model {
    fn to_json(&self) -> Option<Value> {
        serde_json::to_value(self).ok()
    }
}
