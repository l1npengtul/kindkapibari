use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable, Queryable))]
#[cfg_attr(feature = "server", table_name = "user_metadata")]
pub struct UserMetadata {
    pub id: Uuid,
    pub signup_date: chrono::DateTime<Utc>,
}

#[cfg(feature = "server")]
table! {
    user_metadata {
        id -> Uuid,
        signup_date -> Timestampz,
    }
}
