mod api;
mod config;

#[macro_use]
extern crate tokio;
#[macro_use]
extern crate sqlx;

use crate::config::ServerConfig;
use sled::Db;
use sqlx::postgres::PgPoolOptions;
use std::{iter::Once, sync::Arc};
use tokio::{fs::File, io::AsyncReadExt, sync::OnceCell};

pub static SLED_DB: Arc<OnceCell<Db>> = Arc::new(OnceCell::new());

#[tokio::main]
async fn main() {
    // logger
    tracing_subscriber::fmt::init();

    // config
    let mut config_file = File::open("config.toml").await.unwrap();
    let mut toml_str = String::new();
    config_file.read_to_string(&mut toml_str).await.unwrap();
    let config = toml::from_str::<ServerConfig>(&toml_str).unwrap();

    // postgres
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database)
        .await
        .expect("Cannot Connect to Postgres!");

    sqlx::migrate!().run(&db).await?;

    // sled
    let sled_cfg = sled::Config::new().temporary(true);
    SLED_DB
        .set(sled_cfg.open().expect("Failed to open sled database!"))
        .unwrap();
}
