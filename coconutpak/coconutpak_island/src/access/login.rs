use crate::permissions::Scopes::Report;
use crate::{
    permissions::Scopes,
    schema::{api_key, session, user},
    AppData, SResult, ServerError,
};
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use chrono::{TimeZone, Utc};
use color_eyre::Report;
use kindkapibari_core::dbarray::DBArray;
use kindkapibari_core::secret::{check_equality, generate_signed_key};
use kindkapibari_core::{reseedingrng::AutoReseedingRng, snowflake::SnowflakeIdGenerator};
use once_cell::sync::Lazy;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::{fmt::Display, ops::Add, sync::Arc};
use tokio::sync::Mutex;
use tracing::instrument;
use uuid::Uuid;

const AUTH_REDIS_KEY_START_APIKEY: [u8; 3] = *b"cak";
const AUTH_REDIS_KEY_START_SESSION: [u8; 3] = *b"cse";
pub const API_KEY_PREFIX_NO_DASH: &'static str = "A";
pub const TOKEN_PREFIX_NO_DASH: &'static str = "S";
pub const SECRET_SEPARATOR: &'static str = "!";

// 196! 196! 196! 196!
static AUTO_RESEEDING_TOKEN_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static AUTO_RESEEDING_NONCE_RNG: Lazy<Arc<Mutex<AutoReseedingRng<200704>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));
static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

pub type CoconutPakApiKey = String;
pub type CoconutPakSessionToken = String;

const fn uuid_to_byte_array(uuid: Uuid) -> [u8; 16] {
    let u128 = uuid.as_u128();
    u128.to_le_bytes()
}

#[instrument]
pub async fn new_apikey(
    state: Arc<AppData>,
    user_id: u64,
    name: String,
) -> SResult<CoconutPakApiKey> {
    let config = state.config.read().await;
    let key = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let api_key_active = api_key::ActiveModel {
        name: ActiveValue::Set(name),
        owner: ActiveValue::Set(user_id),
        key_hashed: ActiveValue::Set(key.hash),
        salt: ActiveValue::Set(DBArray::from(key.salt)),
        created: ActiveValue::Set(Utc::now()),
        ..Default::default()
    };

    let apikey = format!(
        "{}.{}.{}{}{}",
        base64::encode(key.nonce),
        API_KEY_PREFIX_NO_DASH,
        base64::encode(key.salt),
        SECRET_SEPARATOR,
        base64::encode(key.signed),
    );

    api_key_active.insert(&state.database).await?;
    Ok(apikey)
}

#[instrument]
pub async fn new_session(state: Arc<AppData>, user_id: u64) -> SResult<CoconutPakSessionToken> {
    let config = state.config.read().await;
    let key = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let session_token_active = session::ActiveModel {
        owner: ActiveValue::Set(user_id),
        expire: ActiveValue::Set(Utc::now().add(chrono::Duration::days(69))), // haha funny sex number now laugh
        created: ActiveValue::Set(Utc::now()),
        session_hashed: ActiveValue::Set(key.hash),
        salt: ActiveValue::Set(DBArray::from(key.salt)),
        ..Default::default()
    };

    let session = format!(
        "{}.{}.{}{}{}",
        base64::encode(key.nonce),
        TOKEN_PREFIX_NO_DASH,
        base64::encode(key.salt),
        SECRET_SEPARATOR,
        base64::encode(key.signed),
    );

    session_token_active.insert(&state.database).await?;
    Ok(session)
}

pub async fn generate_id(config: Arc<AppData>) -> Option<u64> {
    ID_GENERATOR.generate_id(config.config.read().await.machine_id)
}

pub enum Authorized {
    ApiKey(u64, Vec<Scopes>),
}

#[instrument]
pub async fn verify_apikey(
    database: Arc<AppData>,
    raw: Vec<u8>,
    salt: Vec<u8>,
) -> SResult<Option<user::Model>> {
    let api_key = api_key::Entity::find()
        .filter(api_key::Column::Salt.eq(&salt))
        .one(&database.database)
        .await?
        .ok_or(ServerError::Unauthorized)?;

    if check_equality(&raw, &api_key.key_hashed, &api_key.salt) {
        Ok(user::Entity::find_by_id(api_key.owner)
            .one(&database.database)
            .await?)
    } else {
        return Err(ServerError::Unauthorized);
    }
}

#[instrument]
pub async fn verify_session(
    database: Arc<AppData>,
    raw: Vec<u8>,
    salt: Vec<u8>,
) -> SResult<Option<user::Model>> {
    let session = session::Entity::find()
        .filter(session::Column::Salt.eq(&salt))
        .filter(session::Column::Expire.gt(Utc::now()))
        .one(&database.database)
        .await?
        .ok_or(ServerError::Unauthorized)?;

    if check_equality(&raw, &session.session_hashed, &session.salt) {
        Ok(user::Entity::find_by_id(session.owner)
            .one(&database.database)
            .await?)
    } else {
        return Err(ServerError::Unauthorized);
    }
}

#[instrument]
pub async fn create_new_user(state: Arc<AppData>, kkb_id: u64) -> SResult<u64> {
    let config = state.config.read().await;
    // make sure this user doesn't exist
    if !user::Entity::find()
        .filter(user::Column::KkbId.eq(kkb_id))
        .one(&state.database)
        .await?
        .is_none()
    {
        return Err(ServerError::InternalServer(Report::msg("already exists")));
    }

    let id = ID_GENERATOR
        .generate_id(config.machine_id)
        .ok_or(ServerError::InternalServer(Report::msg(
            "failed to generate ID",
        )))?;

    let user_active = user::ActiveModel {
        id: ActiveValue::Set(id),
        kkb_id: ActiveValue::Set(kkb_id),
        username: Default::default(),
        restricted_account: Default::default(),
        administrator_account: Default::default(),
        fake_account: Default::default(),
        email: Default::default(),
    };
}
