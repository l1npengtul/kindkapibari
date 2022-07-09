use crate::{access::oauth_thirdparty::AuthorizationProviders, State};
use kindkapibari_core::secret::{
    create_new_token_with_refresh, decode_access_token,
    decode_access_token_without_time_verification, decode_refresh_token, JWTPair, RefreshClaims,
    TokenClaims, TokenType,
};
use kindkapibari_schema::{
    error::ServerError,
    schema::users::{connections, refresh_tokens, user},
    SResult,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter,
};
use std::{borrow::Cow, sync::Arc};
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

#[instrument]
pub async fn generate_login_token(state: Arc<State>, user: u64) -> SResult<JWTPair> {
    let user = user_by_id(state.clone(), user).await?;
    let token_id = state.id_generator.login_token_ids.generate_id();
    let refresh_id = state.id_generator.refresh_token_ids.generate_id();
    let config = state.config.read().await;
    let access_claim = TokenClaims::new()
        .set_user(user.id)
        .set_role(user.roles)
        .set_id(token_id)
        .set_token_type(TokenType::Login)
        .set_machine_id(config.machine_id);

    let mut refresh_claim: RefreshClaims = access_claim.clone().into().set_id(refresh_id);

    let grant_pair = create_new_token_with_refresh(
        &access_claim,
        &refresh_claim,
        config.signing_keys.login_key.as_bytes(),
    )?;

    let refresh_active = refresh_tokens::ActiveModel {
        id: ActiveValue::Set(refresh_id),
        owner: ActiveValue::Set(user.id),
        related: ActiveValue::Set(token_id),
        expire: ActiveValue::Set(refresh_claim.exp as u64),
        created: ActiveValue::Set(refresh_claim.iat as u64),
        revoked: ActiveValue::Set(false),
        stored_secret: ActiveValue::Set(refresh_claim),
    };

    refresh_active.insert(&state.database).await?;

    Ok(grant_pair)
}

#[instrument]
pub async fn verify_user_login_token(state: Arc<State>, token: String) -> SResult<user::Model> {
    let config = state.config.read().await;

    let token = decode_access_token(token, config.signing_keys.login_key.as_bytes())
        .map_err(|_| ServerError::Unauthorized)?;
    if token.token_type != TokenType::Login {
        return Err(ServerError::Forbidden);
    }
    user_by_id(state.clone(), token.user_id)
}

#[instrument]
pub async fn refresh_user_login_token(
    state: Arc<State>,
    access: String,
    refresh: String,
) -> SResult<JWTPair> {
    let config = state.config.read().await;

    let expired_access = decode_access_token_without_time_verification(
        access,
        config.signing_keys.login_key.as_bytes(),
    )
    .map_err(|_| ServerError::Unauthorized)?;
    if expired_access.token_type != TokenType::Login {
        return Err(ServerError::Forbidden);
    }

    let refresh = decode_refresh_token(refresh, config.signing_keys.login_key.as_bytes())
        .map_err(|_| ServerError::Unauthorized)?;
    if refresh.token_type != TokenType::Login
        && refresh.reference_token != expired_access.jti
        && refresh.user_id != expired_access.user_id
    {
        return Err(ServerError::Forbidden);
    }

    let mut db_refreeh_tkn = refresh_tokens::Entity::find_by_id(refresh.jti)
        .one(&state.database)
        .await?
        .ok_or(ServerError::Unauthorized)?
        .into_active_model();
    db_refreeh_tkn.revoked = ActiveValue::Set(true);
    db_refreeh_tkn.update(&state.database).await?;

    generate_login_token(state.clone(), refresh.reference_token)
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
