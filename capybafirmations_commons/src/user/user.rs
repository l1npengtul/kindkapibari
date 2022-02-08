use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable, Queryable))]
#[cfg_attr(feature = "server", table_name = "users")]
pub struct Users {
    pub id: Uuid,
    pub username: String,
    pub handle: String,
}

#[cfg(feature = "server")]
table! {
    users {
        id -> Uuid,
        username -> Text,
        handle -> Text,
    }
}
