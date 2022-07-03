use crate::{access::oauth_thirdparty::AuthorizationProviders, State};
use chrono::{Duration, Utc};
use kindkapibari_core::secret::{GeneratedToken, SentSecret};
use kindkapibari_schema::{
    error::ServerError,
    schema::users::{connections, login_tokens, user},
    SResult,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use std::{borrow::Cow, ops::Add, sync::Arc};
use tracing::instrument;

pub const AUTH_REDIS_KEY_START_SESSION: [u8; 2] = *b"se";
pub const LOGIN_TOKEN_PREFIX_NO_DASH: &str = "LT";

// static AUTO_RESEEDING_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
//     Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

#[instrument]
pub async fn user_by_id(state: Arc<State>, id: u64) -> SResult<user::Model> {
    // check our local cache
    if let Some(possible_user) = state.caches.users_cache.get(&id) {
        return Ok(possible_user);
    }

    match user::Entity::find_by_id(id).one(&state.database).await? {
        Some(u) => {
            state.caches.users_cache.insert(id, u.clone()).await;
            Ok(u)
        }
        None => Err(ServerError::NotFound(Cow::from("user"), Cow::from("id"))),
    }
}

// LOGIN TOKEN CONVENTION: ALL LOGIN TOKENS ARE ENCRYPTED IN REDIS
#[instrument]
pub async fn generate_login_token(state: Arc<State>, user: u64) -> SResult<SentSecret> {
    let user = user_by_id(state.clone(), user).await?;
    let config = state.config.read().await;
    let gen_token = GeneratedToken::new(
        user.id,
        config.signing_keys.login_key.as_bytes(),
        config.machine_id,
    )
    .await
    .map_err(|why| ServerError::InternalServer(why))?;
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
        .insert(gen_token.sent.clone(), user.id)
        .await;

    Ok(gen_token.sent)
}

#[instrument]
pub async fn verify_user_login_token(
    state: Arc<State>,
    token: SentSecret,
) -> SResult<Option<user::Model>> {
    let config = state.config.read().await;
    if let Some(user_id) = state.caches.login_token_cache.get(&token) {
        return Ok(user::Entity::find_by_id(user_id)
            .one(&state.database)
            .await?);
    }

    let user_id = token
        .user_id(config.signing_keys.login_key.as_bytes())
        .ok_or_else(|| ServerError::NotFound(Cow::from("ID not found"), Cow::from("???")))?;

    // yes, we're using lazy loading
    // no, it is not performant
    // yes, someone can probably optimize this.
    // TODO: optimize lazy loaded query into eager query
    // TODO: use openapi to dogfood this

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
    Ok(None)
}

#[instrument]
pub async fn burn_login_token(state: Arc<State>, user: u64, token: SentSecret) -> SResult<()> {
    let config = state.config.read().await;
    if user
        != token
            .user_id(config.signing_keys.login_key.as_bytes())
            .ok_or_else(|| ServerError::BadRequest(Cow::from("bad token")))?
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

// #[instrument]
// async fn generate_redirect_id(state: Arc<State>) -> String {
//     let salt = state.id_generator.redirect_ids.generate_id().to_be_bytes();
//     let rng_gen = AUTO_RESEEDING_RNG.lock().await.generate_bytes::<64>();
//     base64::encode(blake3::hash(&[rng_gen.as_slice(), salt.as_slice()].concat()).as_bytes())
// }

#[allow(clippy::cast_sign_loss)]
#[instrument]
pub async fn detect_user_already_exists_auth_provider(
    state: Arc<State>,
    maybeuser: AuthorizationProviders,
) -> SResult<Option<u64>> {
    // check if the account exists in connections
    let prepared = connections::Entity::find();

    let exists = match &maybeuser {
        AuthorizationProviders::Twitter(twt) => {
            prepared
                .filter(connections::Column::TwitterId.eq(Some(twt.twitter_id)))
                .one(&state.database)
                .await?
        }
        AuthorizationProviders::Github(ghb) => {
            prepared
                .filter(connections::Column::GithubId.eq(Some(ghb.github_id as u64)))
                .one(&state.database)
                .await?
        }
    };

    if let Some(user) = exists {
        return Ok(Some(user.user_id));
    }

    // email account
    let email_chk = match maybeuser {
        AuthorizationProviders::Twitter(twt) => twt.email,
        AuthorizationProviders::Github(ghb) => ghb.email,
    };

    if let Some(email) = email_chk {
        return match user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&state.database)
            .await?
        {
            Some(user) => Ok(Some(user.id)),
            None => Ok(None),
        };
    }

    Ok(None)
}

#[instrument]
pub async fn user_by_username(state: Arc<State>, name: &str) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(name))
        .one(&state.database)
        .await?;
    Ok(user)
}

#[instrument]
pub async fn user_by_email(state: Arc<State>, email: &str) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(&state.database)
        .await?;
    Ok(user)
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
