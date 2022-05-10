#![deny(clippy::pedantic)]
#![warn(clippy::all)]

mod access;
mod api;
mod badges;
mod config;
mod context;
mod login;
mod roles;
mod schema;
mod scopes;

use crate::config::Config;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;

const EPOCH_START: u64 = 1650125769; // haha nice

pub struct AppData {
    redis: ConnectionManager,
    database: DatabaseConnection,
    config: RwLock<Config>,
}

#[tokio::main]
async fn main() {}
