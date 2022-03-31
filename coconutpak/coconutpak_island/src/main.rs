#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![feature(thread_is_running)]

use crate::config::Config;
use argon2::{Algorithm, Argon2, Params, Version};
use once_cell::sync::Lazy;
use redis::aio::ConnectionManager;
use redis::Client;
use std::sync::Arc;

static CONFIG: Lazy<Arc<Config>> = Lazy::new(|| {
    let config = Config::load().unwrap_or_default();
    Arc::new(config)
});
static REDIS: Lazy<Arc<ConnectionManager>> = Lazy::new(|| {
    let client = Client::open(CONFIG.database.redis_url).unwrap();
    Arc::new(ConnectionManager::new(client).unwrap())
});

mod api;
mod coconutpak_cleanup;
mod coconutpak_compiler;
mod config;
mod schema;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
