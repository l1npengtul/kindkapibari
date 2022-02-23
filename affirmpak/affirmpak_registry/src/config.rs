use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub static_location: String,
    pub postgres: PostgresSQL,
    pub github: GithubLogin,
}

impl ServerConfig {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubLogin {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostgresSQL {
    pub url: String,
}
