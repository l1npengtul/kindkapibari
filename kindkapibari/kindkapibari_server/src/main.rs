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
use kindkapibari_core::make_caches;
use kindkapibari_core::secret::DecodedSecret;
use kindkapibari_core::snowflake::SnowflakeIdGenerator;
use moka::future::Cache;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::{io::AsyncReadExt, sync::RwLock};

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

// pub struct Caches {
//     pub users: Cache<u64, Arc<Option<user::Model>>>,
//     pub login_token: Cache<DecodedSecret, Arc<Option<user::Model>>>,
//     pub oauth_token: Cache<DecodedSecret, Arc<Option<users::AuthorizedUser>>>,
//     pub applications: Cache<u64, Arc<Option<applications::Model>>>,
// }

make_caches! {
    users: u64: user::Model,
    login_token: DecodedSecret: user::Model,
    oauth_token: DecodedSecret: users::AuthorizedUser,
    applications: u64: applications::Model
}

#[tokio::main]
async fn main() {}
