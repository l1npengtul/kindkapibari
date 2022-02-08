use crate::pronouns::Pronouns;
use diesel::pg::Pg;
use diesel::serialize::Output;
use diesel::sql_types::Jsonb;
use diesel::types::ToSql;
use diesel::IntoSql;
use std::io::Write;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub theme: Theme,
    pub pronouns: Pronouns,
}

impl ToSql<Jsonb, Pg> for Preferences {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        serde_json::to_value(self).unwrap().to_sql(out)
    }
}
