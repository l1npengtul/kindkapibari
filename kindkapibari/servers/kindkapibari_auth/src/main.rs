#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::perf)]
#![allow(clippy::missing_errors_doc)]

use crate::{config::Config, handlers::signup::PostSignupSent};
use kindkapibari_core::{
    gender::Gender,
    make_caches,
    pronouns::{PronounProfile, Pronouns},
    roles::Role,
    secret::SentSecret,
    snowflake::SnowflakeIdGenerator,
    user_data::{Locale, UserData, UserSignupRequest},
};
use kindkapibari_schema::{
    redis::RedisState,
    schema::users::{user, user::Model},
};
use redis::aio::{ConnectionLike, ConnectionManager};
use sea_orm::DatabaseConnection;
use std::{
    fmt::{Debug, Formatter},
    fs::File,
    io::Write,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};
use tokio::sync::RwLock;
use utoipa::OpenApi;

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

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        handlers (
            handlers::login::login_with_twitter,
            handlers::login::login_with_github,
            handlers::login::verify_login_token,
            handlers::signup::burn_signup_token,
            handlers::signup::signup
        ),
        components(
            Model,
            UserSignupRequest,
            PostSignupSent,
            UserData,
            Pronouns,
            PronounProfile,
            Gender,
            Locale,
            Role,
            SentSecret
        ),
        tags (
            (name = "auth", description = "Authentication/Login/Signup API")
        )
    )]
    struct ApiDoc;

    // write apidoc to file if arg exists
    if std::env::args().any(|x| x == *"--write") {
        let mut to_write = File::create("auth-api.json").unwrap();
        to_write
            .write_all(ApiDoc::openapi().to_pretty_json().unwrap().as_bytes())
            .unwrap();
    }

    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);

    // FIXME: instantiate State

    let routes = handlers::routes();

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
