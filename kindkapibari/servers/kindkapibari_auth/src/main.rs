#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::perf)]
#![allow(clippy::missing_errors_doc)]

use crate::config::Config;
use kindkapibari_core::{make_caches, secret::SentSecret, snowflake::SnowflakeIdGenerator};
use kindkapibari_schema::{redis::RedisState, schema::users::user};
use redis::aio::{ConnectionLike, ConnectionManager};
use sea_orm::DatabaseConnection;
use std::{
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
};
use tokio::sync::RwLock;

pub mod access;
mod config;
pub mod handlers;

#[derive(Clone)]
pub struct RedisCMWithDebug {
    pub redis: ConnectionManager,
}

impl Deref for RedisCMWithDebug {
    type Target = ConnectionManager;

    fn deref(&self) -> &Self::Target {
        &self.redis
    }
}

impl DerefMut for RedisCMWithDebug {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.redis
    }
}

impl Debug for RedisCMWithDebug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_db())
    }
}

#[derive(Debug)]
pub struct State {
    pub redis: RedisCMWithDebug,
    pub database: DatabaseConnection,
    pub config: RwLock<Config>,
    pub caches: Caches,
    pub id_generator: IdGenerators,
}

impl RedisState for State {
    fn redis(&self) -> &ConnectionManager {
        &self.redis.redis
    }
}

#[derive(Debug)]
pub struct IdGenerators {
    pub user_ids: SnowflakeIdGenerator,
    pub redirect_ids: SnowflakeIdGenerator,
}

make_caches! {
    users: u64: user::Model,
    login_token: SentSecret: u64
}

fn main() {
    println!("Hello, world!");
}
