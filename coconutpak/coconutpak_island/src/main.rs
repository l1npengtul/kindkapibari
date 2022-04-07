#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(thread_is_running)]

use crate::config::Config;
use color_eyre::eyre;
use meilisearch_sdk::client::Client;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppData {
    redis: ConnectionManager,
    database: DatabaseConnection,
    meilisearch: Client,
    config: Config,
}

pub type SResult<T> = eyre::Result<T>;

mod api;
mod coconutpak_cleanup;
mod coconutpak_compiler;
mod config;
mod login;
mod report;
mod schema;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
