use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(Insertable, Queryable))]
#[cfg_attr(feature = "server", table_name = "user_connection_info")]
pub struct UserConnectionsInfo {
    pub id: Uuid,
    pub twitter: String,
    pub github: Option<String>,
}

#[cfg(feature = "server")]
table! {
    user_connection_info {
        id -> Uuid,
        twitter -> Text,
        github -> Nullable<Text>,
    }
}
