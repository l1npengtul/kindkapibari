use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::Output;
use diesel::sql_types::Jsonb;
use diesel::types::ToSql;
use semver::Version;
use std::io::Write;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SemVer {
    version: Version,
}

impl Deref for SemVer {
    type Target = Version;

    fn deref(&self) -> &Self::Target {
        &self.version
    }
}

impl DerefMut for SemVer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.version
    }
}

impl ToSql<Jsonb, Pg> for SemVer {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        serde_json::to_value(self).unwrap().to_sql(out)
    }
}

impl FromSql<Jsonb, Pg> for SemVer {
    fn from_sql(bytes: Option<&diesel::backend::RawValue>) -> diesel::deserialize::Result<Self> {
        todo!()
    }
}
