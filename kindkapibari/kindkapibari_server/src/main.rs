#![deny(clippy::pedantic)]
#![warn(clippy::all)]

extern crate core;

mod access;
mod api;
mod badges;
mod config;
mod context;
mod error;
mod login;
mod roles;
mod schema;
mod scopes;

use crate::config::Config;
use crate::error::ServerError;
use color_eyre::Report;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;

const EPOCH_START: u64 = 1650125769; // haha nice
type SResult<T> = Result<T, ServerError>;
type AResult<T> = Result<T, Report>;

pub struct AppData {
    redis: ConnectionManager,
    database: DatabaseConnection,
    config: RwLock<Config>,
}

#[tokio::main]
async fn main() {}
