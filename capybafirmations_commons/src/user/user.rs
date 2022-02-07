use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Model {
    pub user_id: Uuid,
    pub username: String,
    pub handle: String,
}
