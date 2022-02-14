mod api;
mod config;
mod context;

use crate::config::ServerConfig;
use crate::context::ApiContext;
use sea_orm::Database;
use sled::Db;
use std::time::Duration;
use std::{iter::Once, sync::Arc};
use tokio::{fs::File, io::AsyncReadExt, sync::OnceCell};

// #[tokio::main]
async fn main() {
    // logger
    tracing_subscriber::fmt::init();

    // config
    let mut config_file = File::open("config.toml").await.unwrap();
    let mut toml_str = String::new();
    config_file.read_to_string(&mut toml_str).await.unwrap();
    let config = Arc::new(toml::from_str::<ServerConfig>(&toml_str).unwrap());

    // postgres
    let db = Arc::new(
        Database::connect(
            sea_orm::ConnectOptions::new((&config.database).clone())
                .max_connections(100)
                .min_connections(5)
                .connect_timeout(Duration::from_secs(10))
                .idle_timeout(Duration::from_secs(10))
                .sqlx_logging(true),
        )
        .await
        .expect("Database Connection Fail!"),
    );

    // sled
    let sled_cfg = sled::Config::new().temporary(true);
    let sled_cache = Arc::new(sled_cfg.open().expect("Failed to open sled database!"));

    let api_layer_context = ApiContext {
        config,
        database: db,
        cache: sled_cache,
    };

    let app =
}
