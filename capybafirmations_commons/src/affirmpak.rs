use crate::languages::Languages;
use crate::tags::Tags;
use diesel::pg::Pg;
use diesel::serialize::ToSql;
use diesel::sql_types::Jsonb;
use semver::Version;
use url::Url;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable, Queryable))]
#[cfg_attr(feature = "server", table_name = "affirm_pak")]
pub struct AffirmPak {
    pub id: Uuid,
    pub name: String,
    pub author: Uuid,
    pub version: Version,
    pub language: Option<Languages>,
    pub tags: Vec<Tags>,
    pub downloads: u64,
    pub likes: u64,
    pub source: String,
    pub data: String,
}

#[cfg(feature = "server")]
table! {
    affirm_pak {
        id: Uuid,
        name: Text,
        author: Uuid,
        version
    }
}
