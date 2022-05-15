use crate::access::TOKEN_SEPERATOR;
use crate::{
    access::insert_into_cache,
    roles::Roles,
    schema::users::{login_tokens, passwords},
    user, AppData, SResult, ServerError,
};
use argon2::{Algorithm, Argon2, Params, Version};
use chrono::{DateTime, Duration, TimeZone, Utc};
use kindkapibari_core::secret::{check_equality, decode_gotten_secret, DecodedSecret};
use kindkapibari_core::{
    dbarray::DBArray, dbvec::DBVec, secret::generate_signed_key, snowflake::SnowflakeIdGenerator,
};
use once_cell::sync::Lazy;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, FromQueryResult, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};
use std::ops::Add;
use std::sync::Arc;
use tracing::instrument;

pub const AUTH_REDIS_KEY_START_SESSION: [u8; 2] = *b"se";
pub const LOGIN_TOKEN_PREFIX_NO_DASH: &'static str = "LT";

static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

// LOGIN TOKEN CONVENTION: ALL LOGIN TOKENS ARE ENCRYPTED IN REDIS
#[instrument]
pub async fn generate_login_token(state: Arc<AppData>, user: user::Model) -> SResult<String> {
    let config = state.config.read().await;
    let key = generate_signed_key(config.machine_id, config.signing_key.as_bytes())
        .map_err(|why| ServerError::InternalServer(why))?;

    let token = format!(
        "{}.{}.{}{}{}",
        base64::encode(key.nonce),
        LOGIN_TOKEN_PREFIX_NO_DASH,
        base64::encode(key.salt),
        TOKEN_SEPERATOR,
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

    // cache
    tokio::task::spawn(async {
        // insert into redis
        insert_into_cache(state.clone(), &token, &user, None);
        // insert into moka
        state.caches.login_token.insert(token.clone(), user);
    });

    Ok(token)
}

#[instrument]
pub async fn verify_login_token(state: Arc<AppData>, token: DecodedSecret) -> SResult<user::Model> {
    let login_token = login_tokens::Entity::find()
        .filter(login_tokens::Column::Salt.eq(&token.salt))
        .filter(login_tokens::Column::Expire.gt(Utc::now()))
        .one(&state.database)
        .await?
        .ok_or(ServerError::Unauthorized)?;

    if check_equality(&token.raw, &login_token.session_hashed, &login_token.salt) {
        Ok(user::Entity::find_by_id(login_token.owner)
            .one(&state.database)
            .await?
            .ok_or(ServerError::Unauthorized)?)
    } else {
        return Err(ServerError::Unauthorized);
    }
}

// TODO: Add 2FA support
#[instrument]
pub async fn verify_username_passwd(
    state: Arc<AppData>,
    username: String,
    password: String,
) -> SResult<String> {
    #[derive(FromQueryResult)]
    struct UserAndPasswordModel {
        pub id: u64,
        pub username: String,
        pub handle: String,
        pub email: Option<String>,
        pub profile_pictures: Option<String>,
        pub creation_date: DateTime<Utc>,
        pub password_id: u64,
        pub roles: DBVec<Roles>,
        pub last_changed: DateTime<Utc>,
        pub password_hashed: Vec<u8>,
        pub salt: DBArray<u8, 32>,
    }

    let user_auth = user::Entity::find()
        .filter(user::Column::Handle.eq(&username))
        .join(JoinType::Join, passwords::Relation::User.def())
        .group_by(user::Column::Id)
        .column_as(passwords::Column::Id, "password_id")
        .into_model::<UserAndPasswordModel>()
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

    let mut user_input_hash_out = Vec::with_capacity(64);
    argon2_key.hash_password_into(
        password.as_bytes(),
        user_auth.salt.as_bytes(),
        &mut user_input_hash_out,
    )?;

    // create a new login token
    if user_input_hash_out == user_auth.password_hashed {
        Ok(generate_login_token(
            state,
            user::Model {
                id: user_auth.id,
                username: user_auth.username,
                handle: user_auth.handle,
                email: user_auth.email,
                profile_picture: user_auth.profile_pictures,
                creation_date: user_auth.creation_date,
                roles: user_auth.roles,
            },
        )?)
    } else {
        Err(ServerError::Unauthorized)
    }
}
