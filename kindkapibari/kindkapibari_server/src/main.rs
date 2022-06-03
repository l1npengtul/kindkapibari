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

use crate::schema::{applications, users};
use crate::{config::Config, error::ServerError, schema::users::user, scopes::KKBScope};
use color_eyre::Report;
use kindkapibari_core::secret::DecodedSecret;
use moka::future::Cache;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use tokio::{io::AsyncReadExt, sync::RwLock};
use kindkapibari_core::snowflake::SnowflakeIdGenerator;

const EPOCH_START: u64 = 1650125769; // haha nice
type SResult<T> = Result<T, ServerError>;
type AResult<T> = Result<T, Report>;

pub const THIS_SITE_URL: &'static str = "https://kindkapibari.land";

pub struct AppData {
    pub redis: ConnectionManager,
    pub database: DatabaseConnection,
    pub config: RwLock<Config>,
    pub caches: Caches,
    pub id_generator: SnowflakeIdGenerator,
}

pub struct Caches {
    pub users: Cache<u64, Option<user::Model>>,
    pub login_token: Cache<DecodedSecret, Option<user::Model>>,
    pub oauth_token: Cache<DecodedSecret, Option<users::AuthorizedUser>>,
    pub applications: Cache<u64, Option<applications::Model>>,
}

#[tokio::main]
async fn main() {}
