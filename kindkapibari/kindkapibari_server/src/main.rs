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

use crate::schema::users;
use crate::{config::Config, error::ServerError, schema::users::user, scopes::Scope};
use color_eyre::Report;
use kindkapibari_core::secret::DecodedSecret;
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
    pub login_token: Cache<DecodedSecret, user::Model>,
    pub oauth_token: Cache<DecodedSecret, users::AuthorizedUser>,
}

#[tokio::main]
async fn main() {}
