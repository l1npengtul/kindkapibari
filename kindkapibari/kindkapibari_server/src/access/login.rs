use crate::schema::users::{login_tokens, passwords};
use crate::{user, AResult, AppData, SResult, ServerError};
use argon2::{Algorithm, Argon2, Params, Version};
use chrono::{Duration, TimeZone, Utc};
use kindkapibari_core::dbarray::DBArray;
use kindkapibari_core::secret::generate_signed_key;
use kindkapibari_core::snowflake::SnowflakeIdGenerator;
use once_cell::sync::Lazy;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::ops::Add;
use std::sync::Arc;

const AUTH_REDIS_KEY_START_SESSION: [u8; 2] = *b"se";
const LOGIN_TOKEN_PREFIX_NO_DASH: &'static str = "LT";

static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

pub async fn generate_login_token(state: Arc<AppData>, user_id: u64) -> AResult<String> {
    let config = state.config.read().await;
    let key = generate_signed_key(config.machine_id, config.signing_key.as_bytes())?;

    let token = format!(
        "{}.{}.{}-{}",
        base64::encode(key.nonce),
        LOGIN_TOKEN_PREFIX_NO_DASH,
        base64::encode(&key.salt),
        base64::encode(&key.signed)
    );

    let now = Utc::now();

    let login_token_active = login_tokens::ActiveModel {
        owner: ActiveValue::Set(user_id),
        expire: ActiveValue::Set(now.add(Duration::days(69))),
        created: ActiveValue::Set(now),
        session_hashed: ActiveValue::Set(key.signed),
        salt: ActiveValue::Set(DBArray::from(key.salt)),
        ..Default::default()
    };
    login_token_active.insert(&state.database).await?;

    Ok(token)
}

pub async fn log_user_in_username_passwd(
    state: Arc<AppData>,
    username: String,
    password: String,
) -> SResult<String> {
    let password = user::Entity::find()
        .filter(user::Column::Handle.eq(&username))
        .join(JoinType::RightJoin, passwords::Relation::User.def())
        .into_model::<passwords::Model>()
        .one(&state.database)
        .await?
        .ok_or(ServerError::Unauthorized)?;

    let argon2_key = Argon2::new(
        Algorithm::Argon2id,
        Version::default(),
        Params::new(
            Params::DEFAULT_M_COST,
            Params::DEFAULT_T_COST,
            Params::DEFAULT_P_COST,
            Some(64),
        )?,
    );
}
