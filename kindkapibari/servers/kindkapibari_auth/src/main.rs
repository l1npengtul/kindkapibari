#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::perf)]

use crate::config::Config;
use kindkapibari_core::make_caches;
use kindkapibari_core::snowflake::SnowflakeIdGenerator;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

pub mod access;
mod config;
pub mod handlers;

pub struct State {
    pub redis: ConnectionManager,
    pub database: DatabaseConnection,
    pub config: RwLock<Config>,
    pub caches: Caches,
    pub id_generator: IdGenerators,
}

pub struct IdGenerators {
    user_ids: SnowflakeIdGenerator,
    redirect_ids: SnowflakeIdGenerator,
}

make_caches! {
    users: u64: user::Model,
    login_token: SentSecret: u64,
    access_tokens: SentSecret: oauth_authorizations::Model,
    applications: u64: applications::Model
}

fn main() {
    println!("Hello, world!");
}
