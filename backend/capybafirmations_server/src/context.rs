use crate::ServerConfig;
use sea_orm::DatabaseConnection;
use sled::Db;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<ServerConfig>,
    pub database: Arc<DatabaseConnection>,
    pub cache: Arc<Db>,
}
