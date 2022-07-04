#![deny(clippy::pedantic)]
#![warn(clippy::all)]

extern crate core;

pub mod access;
mod api;
mod config;

use crate::{
    config::Config,
    schema::{
        applications, users,
        users::{oauth_authorizations, user},
    },
};
use color_eyre::Report;
use kindkapibari_core::{make_caches, scopes::KKBScope, snowflake::SnowflakeIdGenerator};
use kindkapibari_schema::{
    appdata_traits::{
        AppData, AppDataCache, AppDataDatabase, AppDataKeyTypes, AppDataRedis, AppDataSigningKey,
    },
    error::ServerError,
    schema::users::user,
};
use moka::future::Cache;
use once_cell::sync::OnceCell;
use oxide_auth_async::primitives::{Authorizer, Issuer, Registrar};
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::{io::AsyncReadExt, sync::RwLock};

const EPOCH_START: u64 = 1650125769; // haha nice

pub const THIS_SITE_URL: &'static str = "https://kindkapibari.land";

pub static SERVERSTATE: OnceCell<Arc<State>> = OnceCell::new();

// pub struct OAuthState {
//     registrar: Mutex<OAuthRegistrar>,
//     authorizer: Mutex<OAuthAuthorizer>,
//     issuer: Mutex<OAuthIssuer>,
// }
//
// impl OAuthState {
//     pub async fn endpoint(
//         &self,
//     ) -> Generic<impl Registrar + '_, impl Authorizer + '_, impl Issuer + '_> {
//         Generic {
//             registrar: self.registrar.lock().await,
//             authorizer: self.authorizer.lock().await,
//             issuer: self.issuer.lock().await,
//             solicitor: Vacant,
//             scopes: Vacant,
//             response: Vacant,
//         }
//     }
// }

#[derive(Clone, Debug)]
pub struct State {
    pub redis: ConnectionManager,
    pub database: DatabaseConnection,
    pub config: RwLock<Config>,
    pub caches: Caches,
    pub id_generator: IdGenerators,
}

#[derive(Clone, Debug)]
pub struct IdGenerators {
    user_ids: SnowflakeIdGenerator,
    redirect_ids: SnowflakeIdGenerator,
    sober_ids: SnowflakeIdGenerator,
    onetime_reminder_ids: SnowflakeIdGenerator,
}

make_caches! {
    users: u64: user::Model,
    login_token: SentSecret: u64,
    access_tokens: SentSecret: oauth_authorizations::Model,
    applications: u64: applications::Model
}

impl AppData for State {}

impl AppDataRedis for State {
    fn redis(&self) -> &ConnectionManager {
        &self.redis
    }
}

impl AppDataDatabase for State {
    fn database(&self) -> &DatabaseConnection {
        &self.database
    }
}

impl AppDataCache<u64, user::Model> for State {
    fn cache<K, V>(&self) -> &Cache<K, V> {
        &self.caches.users_cache
    }
}

impl AppDataSigningKey for State {
    async fn get_key(&self, key: AppDataKeyTypes) -> Option<&[u8]> {
        match key {
            AppDataKeyTypes::LOGIN => {
                Some(self.config.read().await.signing_keys.login_key.as_bytes())
            }
            AppDataKeyTypes::OAUTH => {
                Some(self.config.read().await.signing_keys.oauth_key.as_bytes())
            }
            AppDataKeyTypes::OTHER => None,
        }
    }
}

#[tokio::main]
async fn main() {}
