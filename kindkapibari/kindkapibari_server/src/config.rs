use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub database: String,
    pub binding: String,
    pub twitter_client_id: String,
    pub twitter_client_secret: String,
}
