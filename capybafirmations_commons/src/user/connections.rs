use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct UserConnectionsInfo {
    pub user_id: Uuid,
    pub twitter: String,
    pub github: Option<String>,
}
