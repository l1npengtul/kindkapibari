use kindkapybari_core::{
    dbarray::DBArray, dbvec::DBVec, manifest::CoconutPakManifest, version::Version,
};
use oauth2::url::Url;
use poem_openapi::registry::{MetaSchema, MetaSchemaRef};
use poem_openapi::types::{ToJSON, Type};
use sea_orm::strum::IntoEnumIterator;
use sea_orm::{
    prelude::{DeriveEntityModel, EntityTrait, PrimaryKeyTrait, Related, RelationTrait},
    ActiveModelBehavior, IdenStatic, RelationDef,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "coconutpak_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub coconutpak: Uuid,
    // manifest
    pub author: DBVec<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: String,
    pub edition: Version,
    #[sea_orm(column_type = "Text", nullable)]
    pub license: String,
    pub version: Version,
    #[sea_orm(column_type = "Text", nullable)]
    pub source: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub readme: String,
    #[sea_orm(nullable)]
    pub tags: DBArray<String, 5>,
    #[sea_orm(column_type = "Text", nullable)]
    pub docs: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub homepage: Option<String>,
    #[sea_orm(nullable)]
    pub categories: DBArray<String, 5>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    CoconutPak,
    CoconutPakData,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::CoconutPak => Entity::belongs_to(super::coconutpak::Entity)
                .from(Column::Coconutpak)
                .to(super::coconutpak::Column::Id)
                .into(),
            Relation::CoconutPakData => Entity::has_one(super::coconutpak_data::Entity).into(),
        }
    }
}

impl Related<super::coconutpak::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPak.def()
    }
}

impl Related<super::coconutpak_data::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoconutPakData.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for CoconutPakManifest {
    fn from(m: Model) -> Self {
        CoconutPakManifest {
            author: m.author.into(),
            name: m.name,
            edition: m.edition,
            license: m.license,
            version: m.version,
            source: m.source.into(),
            description: m.description,
            readme: m.readme,
            tags: m.tags.into(),
            docs: m.docs.map(|x| Url::parse(&x).unwrap_or_default()),
            homepage: m.homepage.map(|x| Url::parse(&x).unwrap_or_default()),
            categories: m.categories.into(),
        }
    }
}

impl Type for Model {
    const IS_REQUIRED: bool = false;
    type RawValueType = Self;
    type RawElementValueType = Self;

    fn name() -> Cow<'static, str> {
        Cow::Borrowed("CoconutPakHistory")
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
