use moka::future::Cache;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use std::hash::Hash;

pub trait AppData: Clone {}

pub trait AppDataRedis: AppData {
    fn redis(&self) -> &ConnectionManager;
}

pub trait AppDataDatabase: AppData {
    fn database(&self) -> &DatabaseConnection;
}

pub trait AppDataCache<K, V>: AppData
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn cache<K, V>(&self) -> &Cache<K, V>;
}

pub trait AppDataIdGenerator: AppData {
    fn generate_id(&self) -> u64;
}

pub enum AppDataKeyTypes {
    LOGIN,
    OAUTH,
    OTHER,
}

#[async_trait::async_trait]
pub trait AppDataSigningKey: AppData {
    async fn get_key(&self, key: AppDataKeyTypes) -> Option<&[u8]>;
}
