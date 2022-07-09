#![deny(clippy::pedantic)]
#![warn(clippy::all)]

pub mod access;
mod api;
mod config;

use crate::{
    api::user::{onetime, recurring, sober, users},
    config::Config,
};
use kindkapibari_core::{
    gender::Gender,
    make_caches,
    pronouns::{PronounProfile, Pronouns},
    reminder::{OneTimeReminder, OneTimeReminders, RecurringReminder, RecurringReminders},
    roles::Role,
    secret::JWTPair,
    snowflake::SnowflakeIdGenerator,
    sober::{Sober, Sobers},
    user_data::{Locale, UserData, UserSignupRequest},
};
use kindkapibari_schema::{error::ServerError, redis::RedisState, schema::users::user::Model};
use once_cell::sync::OnceCell;
use redis::{
    aio::{ConnectionLike, ConnectionManager},
    Client,
};
use sea_orm::{Database, DatabaseConnection};
use std::{
    fmt::{Debug, Formatter},
    net::SocketAddr,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tokio::{fs::File, io::AsyncWriteExt, sync::RwLock};
use utoipa::{
    openapi::{
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        ComponentsBuilder,
    },
    Modify, OpenApi,
};

const EPOCH_START: u64 = 1_650_125_769; // haha nice

pub const THIS_SITE_URL: &str = "https://kindkapibari.land";

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

#[derive(Debug)]
pub struct IdGenerators {
    user_ids: SnowflakeIdGenerator,
    redirect_ids: SnowflakeIdGenerator,
    sober_ids: SnowflakeIdGenerator,
    onetime_reminder_ids: SnowflakeIdGenerator,
    recurring_reminder_ids: SnowflakeIdGenerator,
}

impl RedisState for State {
    fn redis(&self) -> &ConnectionManager {
        &self.redis.redis
    }
}

make_caches! {
    users: u64 : Model
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        handlers(
            onetime::get_user_onetime_reminders,
            onetime::patch_update_onetime_reminders,
            onetime::post_add_onetime_reminder,
            onetime::delete_user_onetime_reminder,
            recurring::get_user_recurring_reminders,
            recurring::patch_update_recurring_reminders,
            recurring::post_add_recurring_reminder,
            recurring::delete_user_recurring_reminder,
            sober::get_user_sobers,
            sober::patch_user_sober_reset_time,
            sober::patch_update_sober,
            sober::post_add_sober,
            sober::delete_user_sober,
            users::username,
            users::user_id,
            users::profile_picture,
            users::get_account_creation_date,
            users::get_user_data,
            users::patch_set_user_data
        ),
        components(
            Model,
            JWTPair,
            UserSignupRequest,
            UserData,
            Pronouns,
            PronounProfile,
            Gender,
            Locale,
            Role,
            OneTimeReminder,
            OneTimeReminders,
            RecurringReminder,
            RecurringReminders,
            Sober,
            Sobers,
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            openapi.components = Some(
                ComponentsBuilder::new()
                    .security_scheme(
                        "api_jwt_token",
                        SecurityScheme::Http(
                            HttpBuilder::new()
                                .scheme(HttpAuthScheme::Bearer)
                                .bearer_format("JWT")
                                .build(),
                        ),
                    )
                    .build(),
            );
        }
    }

    // write apidoc to file if arg exists
    if std::env::args().any(|x| x == *"--write") {
        let mut to_write = File::create("auth-api.json").await.unwrap();
        to_write
            .write_all(ApiDoc::openapi().to_pretty_json().unwrap().as_bytes())
            .await
            .unwrap();
    }

    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);

    // FIXME: instantiate State

    let config = Config::load().expect("Failed to read config");
    let database: DatabaseConnection = Database::connect(&config.database.postgres_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    let redis = RedisCMWithDebug {
        redis: ConnectionManager::new(
            Client::open(config.database.redis_url.clone()).expect("Failed to connect to Redis"),
        )
        .await
        .expect("Failed to open Redis ConnectionManager"),
    };

    let routes = api::user::routes();

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
