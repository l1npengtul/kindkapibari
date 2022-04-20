use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;

pub trait State: Clone + Sync + Send {
    fn redis(&self) -> ConnectionManager;
    fn sea_orm(&self) -> DatabaseConnection;
}
