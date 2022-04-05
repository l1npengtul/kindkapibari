#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(thread_is_running)]

use crate::config::Config;
use argon2::{Algorithm, Argon2, Params, Version};
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use once_cell::sync::Lazy;
use redis::aio::ConnectionManager;
use redis::Client;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

static CONFIG: Lazy<Arc<Config>> = Lazy::new(|| {
    let config = Config::load().unwrap_or_default();
    Arc::new(config)
});

struct AppData {
    redis: ConnectionManager,
    database: DatabaseConnection,
}

mod api;
mod coconutpak_cleanup;
mod coconutpak_compiler;
mod config;
mod login;
mod schema;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
