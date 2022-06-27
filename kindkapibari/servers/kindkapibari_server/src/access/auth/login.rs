use crate::access::user::user_by_id;
use crate::{
    users::{login_tokens, user},
    AppData, SResult, ServerError,
};
use chrono::{Duration, Utc};
use kindkapibari_core::secret::{GeneratedToken, SentSecret};
use redis::AsyncCommands;
use sea_orm::{ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use std::borrow::Cow;
use std::{ops::Add, sync::Arc};
use tracing::instrument;

pub const AUTH_REDIS_KEY_START_SESSION: [u8; 2] = *b"se";
pub const LOGIN_TOKEN_PREFIX_NO_DASH: &'static str = "LT";

// LOGIN TOKEN CONVENTION: ALL LOGIN TOKENS ARE ENCRYPTED IN REDIS
#[instrument]
pub async fn generate_login_token(state: Arc<AppData>, user: u64) -> SResult<SentSecret> {
    let user = user_by_id(state.clone(), user).await?;
    let config = state.config.read().await;
    let gen_token = GeneratedToken::new(user.id, config.signing_keys.login_key.as_bytes())?;
    let login_token_active = login_tokens::ActiveModel {
        id: ActiveValue::NotSet,
        owner: ActiveValue::Set(user.id),
        expire: ActiveValue::Set(Utc::now().add(Duration::days(1337))),
        created: ActiveValue::Set(Utc::now()),
        stored_secret: ActiveValue::Set(gen_token.store),
    };

    login_token_active.insert(&state.database).await?;
    state
        .caches
        .login_token_cache
        .insert(gen_token.sent.clone(), Some(user.id))
        .await;

    Ok(gen_token.sent)
}

#[instrument]
pub async fn verify_login_token(
    state: Arc<AppData>,
    token: SentSecret,
) -> SResult<Option<user::Model>> {
    let config = *state.config.read().await;
    if let Some(user_id) = state.caches.login_token_cache.get(&token) {
        return match user_id {
            Some(id) => Ok(user::Entity::find_by_id(id).one(&state.database).await?),
            None => Ok(None),
        };
    }

    let user_id = token
        .user_id(config.signing_keys.login_key.as_bytes())
        .ok_or(ServerError::NotFound(
            Cow::from("ID not found"),
            Cow::from("???"),
        ))?;

    // yes, we're using lazy loading
    // no, it is not performant
    // yes, someone can probably optimize this.
    // TODO: optimize lazy loaded query into eager query

    let user = user_by_id(state.clone(), user_id).await?;
    // intellij has no idea what the fuck we are doing here, so this typeanno is for that.
    let stored_tokens: Vec<login_tokens::Model> = user
        .find_related(login_tokens::Entity)
        .all(&state.database)
        .await?;

    for st in stored_tokens {
        if st
            .stored_secret
            .verify(&token, config.signing_keys.login_key.as_bytes())
        {
            return Ok(Some(user));
        }
    }
    return Ok(None);
}

#[instrument]
pub async fn burn_login_token(state: Arc<AppData>, user: u64, token: SentSecret) -> SResult<()> {
    let config = state.config.read().await;
    if user
        != token
            .user_id(config.signing_keys.login_key.as_bytes())
            .ok_or(ServerError::BadRequest(Cow::from("bad token")))?
    {
        return Err(ServerError::BadRequest(Cow::from("bad user")));
    }

    let tokens = login_tokens::Entity::find()
        .filter(login_tokens::Column::Owner.eq(user))
        .all(&state.database)
        .await?;
    for server_tkn in tokens {
        if server_tkn
            .stored_secret
            .verify(&token, config.signing_keys.login_key.as_bytes())
        {
            // delete this one
            login_tokens::Entity::delete_by_id(server_tkn.id)
                .exec(&state.database)
                .await?;
            return Ok(());
        }
    }

    Err(ServerError::ISErr(Cow::from("no token")))
}

// // TODO: Add 2FA support
// // TODO: Actually do this (i am lazy)
// #[instrument]
// pub async fn verify_username_passwd(
//     state: Arc<AppData>,
//     username: String,
//     password: String,
// ) -> SResult<String> {
//     #[derive(FromQueryResult)]
//     struct UserAndPasswordModel {
//         pub id: u64,
//         pub username: String,
//         pub handle: String,
//         pub email: Option<String>,
//         pub profile_pictures: Option<String>,
//         pub creation_date: DateTime<Utc>,
//         pub password_id: u64,
//         pub roles: DBVec<Roles>,
//         pub last_changed: DateTime<Utc>,
//         pub password_hashed: Vec<u8>,
//         pub salt: DBArray<u8, 32>,
//     }
//
//     let user_auth = user::Entity::find()
//         .filter(user::Column::Handle.eq(&username))
//         .join(JoinType::Join, passwords::Relation::User.def())
//         .group_by(user::Column::Id)
//         .column_as(passwords::Column::Id, "password_id")
//         .into_model::<UserAndPasswordModel>()
//         .one(&state.database)
//         .await?
//         .ok_or(ServerError::Unauthorized)?;
//
//     let argon2_key = Argon2::new(
//         Algorithm::Argon2id,
//         Version::default(),
//         Params::new(
//             Params::DEFAULT_M_COST,
//             Params::DEFAULT_T_COST,
//             Params::DEFAULT_P_COST,
//             Some(64),
//         )?,
//     );
//
//     let mut user_input_hash_out = Vec::with_capacity(64);
//     argon2_key.hash_password_into(
//         password.as_bytes(),
//         user_auth.salt.as_bytes(),
//         &mut user_input_hash_out,
//     )?;
//
//     // create a new login token
//     if user_input_hash_out == user_auth.password_hashed {
//         generate_login_token(
//             state,
//             user::Model {
//                 id: user_auth.id,
//                 username: user_auth.username,
//                 handle: user_auth.handle,
//                 email: user_auth.email,
//                 profile_picture: user_auth.profile_pictures,
//                 creation_date: user_auth.creation_date,
//                 roles: user_auth.roles,
//             },
//         )
//         .await
//     } else {
//         Err(ServerError::Unauthorized)
//     }
// }
