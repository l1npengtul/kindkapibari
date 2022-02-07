use uuid::Uuid;
use sqlx::FromRow;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct UserMetadata {
    pub user_id: Uuid,
    pub paks: Vec<Uuid>,
}
