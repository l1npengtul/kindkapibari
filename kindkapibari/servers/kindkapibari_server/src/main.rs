#![deny(clippy::pedantic)]
#![warn(clippy::all)]

extern crate core;

mod access;
mod api;
mod badges;
mod config;
mod error;
mod roles;
mod schema;
mod scopes;

use crate::access::auth::oauth::{OAuthAuthorizer, OAuthIssuer, OAuthRegistrar};
use crate::{
    config::Config,
    error::ServerError,
    schema::{
        applications, users,
        users::{oauth_authorizations, user},
    },
    scopes::KKBScope,
};
use color_eyre::Report;
use kindkapibari_core::{make_caches, secret::SentSecret, snowflake::SnowflakeIdGenerator};
use once_cell::sync::OnceCell;
use oxide_auth::frontends::simple::endpoint::{Generic, Vacant};
use oxide_auth_async::primitives::{Authorizer, Issuer, Registrar};
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::{
    io::AsyncReadExt,
    sync::{Mutex, RwLock},
};

const EPOCH_START: u64 = 1650125769; // haha nice
type SResult<T> = Result<T, ServerError>;
type AResult<T> = Result<T, Report>;

pub const THIS_SITE_URL: &'static str = "https://kindkapibari.land";

pub static SERVERSTATE: OnceCell<Arc<AppData>> = OnceCell::new();

pub struct OAuthState {
    registrar: Mutex<OAuthRegistrar>,
    authorizer: Mutex<OAuthAuthorizer>,
    issuer: Mutex<OAuthIssuer>,
}

impl OAuthState {
    pub async fn endpoint(
        &self,
    ) -> Generic<impl Registrar + '_, impl Authorizer + '_, impl Issuer + '_> {
        Generic {
            registrar: self.registrar.lock().await,
            authorizer: self.authorizer.lock().await,
            issuer: self.issuer.lock().await,
            solicitor: Vacant,
            scopes: Vacant,
            response: Vacant,
        }
    }
}

pub struct AppData {
    pub redis: ConnectionManager,
    pub database: DatabaseConnection,
    pub config: RwLock<Config>,
    pub caches: Caches,
    pub id_generator: IdGenerators,
    pub oauth: OAuthState,
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

#[tokio::main]
async fn main() {}
