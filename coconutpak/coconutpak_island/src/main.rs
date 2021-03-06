#![deny(clippy::pedantic)]
#![warn(clippy::all)]

use crate::config::Config;
use crate::error::ServerError;
use color_eyre::eyre;
use meilisearch_sdk::client::Client;
use poem::http::StatusCode;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

const EPOCH_START: u64 = 1650125769; // haha nice

pub struct AppData {
    redis: ConnectionManager,
    database: DatabaseConnection,
    meilisearch: Client,
    config: RwLock<Config>,
}

pub type SResult<T> = Result<T, ServerError>;
pub type EResult<T> = Result<T, StatusCode>;

mod access;
mod api;
mod coconutpak_cleanup;
mod coconutpak_compiler;
mod config;
mod error;
mod login;
mod permissions;
mod report;
mod schema;
mod suspended;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
