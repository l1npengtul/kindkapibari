#![deny(clippy::pedantic)]
#![warn(clippy::all)]

extern crate core;

mod access;
mod api;
mod badges;
mod config;
mod error;
mod login;
mod roles;
mod schema;
mod scopes;

use crate::{config::Config, error::ServerError, schema::users::user};
use color_eyre::Report;
use moka::future::Cache;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::{io::AsyncReadExt, sync::RwLock};

const EPOCH_START: u64 = 1650125769; // haha nice
type SResult<T> = Result<T, ServerError>;
type AResult<T> = Result<T, Report>;

pub struct AppData {
    pub redis: ConnectionManager,
    pub database: DatabaseConnection,
    pub config: RwLock<Config>,
    pub caches: Caches,
}

pub struct Caches {
    pub user_login_token: Cache<String, user::Model>,
    pub user_oauth_token: Cache<String, user::Model>,
}

#[tokio::main]
async fn main() {}
