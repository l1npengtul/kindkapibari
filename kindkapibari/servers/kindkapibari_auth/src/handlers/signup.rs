use crate::{
    access::{
        login::{
            detect_user_already_exists_auth_provider, generate_login_token, user_by_email,
            user_by_id, user_by_username,
        },
        oauth_thirdparty::{get_oauth_client, get_user_data, AuthProviderDataCommon, OAuthAttempt},
    },
    State,
};
use axum::{
    extract::Query,
    routing::{delete, post},
    Extension, Json,
};
use chrono::Utc;
use kindkapibari_core::{roles::Role, route, secret::JWTPair, user_data::UserSignupRequest};
use kindkapibari_schema::{
    error::ServerError,
    redis::{check_if_exists_cache, delet_dis, insert_into_cache, read_from_cache},
    schema::users::{user, userdata},
    SResult,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier};
use sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;
use utoipa::Component;

// u c pp
// i c pp
// we all c pp
// pee with friends :) vs pee alone :C
pub const REDIS_USER_CREATION_PENDING_PREFIX: &str = "ucpp";

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Serialize, Deserialize, Component)]
pub enum RedirectedUser {
    AlreadyExists(JWTPair),
    NewUserCreation {
        slip: String,
        suggested_username: Option<String>,
    },
}

#[derive(Debug, Deserialize, Component)]
pub struct StateAndCode {
    pub state: String,
    pub code: String,
}

#[instrument]
pub async fn redirect(
    Extension(app): Extension<Arc<State>>,
    state_and_code: Query<StateAndCode>,
) -> SResult<Json<RedirectedUser>> {
    let oauth_attempt = read_from_cache::<OAuthAttempt>(app.clone(), &state_and_code.state).await?;
    let config = app.config.read().await.clone();
    if state_and_code.state != oauth_attempt.pkce_verifier() {
        return Err(ServerError::BadRequest(Cow::Borrowed("Bad State")));
    }

    let client_rebuild = match oauth_attempt.authorizer() {
        "twitter" => get_oauth_client(
            config.oauth.twitter.authorize_url,
            config.oauth.twitter.token_url,
            format!("{}/redirect", config.host_url),
            config.oauth.twitter.client_id,
            config.oauth.twitter.secret,
        )
        .map_err(|why| ServerError::InternalServer(why))?,
        "github" => get_oauth_client(
            config.oauth.github.authorize_url,
            config.oauth.github.token_url,
            format!("{}/redirect", config.host_url),
            config.oauth.github.client_id,
            config.oauth.github.secret,
        )
        .map_err(|why| ServerError::InternalServer(why))?,
        _ => return Err(ServerError::BadRequest(Cow::Borrowed("Bad Authorizer"))),
    };

    let token_result = client_rebuild
        .exchange_code(AuthorizationCode::new(state_and_code.code.clone()))
        .set_pkce_verifier(PkceCodeVerifier::new(
            oauth_attempt.pkce_verifier().to_string(),
        ))
        .request_async(async_http_client)
        .await
        .map_err(|why| ServerError::InternalServer(why.into()))?;

    let user_info = get_user_data(oauth_attempt.authorizer(), token_result).await?;
    let maybe_existing_user =
        detect_user_already_exists_auth_provider(app.clone(), user_info.clone()).await?;
    let user_info_common: AuthProviderDataCommon = user_info.into();
    Ok(Json(match maybe_existing_user {
        Some(existing) => {
            RedirectedUser::AlreadyExists(generate_login_token(app.clone(), existing).await?)
        }
        None => {
            // in this case we create a "slip" that the user can trade for not making this request again
            let user_info_num = format!(
                "{}{:?}{}{}",
                user_info_common.id,
                &user_info_common.email.as_ref(),
                &user_info_common.profile_picture,
                &user_info_common.username
            );
            // check if username exists
            let suggested = if user_by_username(app.clone(), &user_info_num)
                .await?
                .is_none()
            {
                None
            } else {
                Some(user_info_common.username.clone())
            };
            // we dont really need this to be cryptographic
            // by the time this is reversed it will already be useless
            let data_hashed = base64::encode(blake3::hash(user_info_num.as_bytes()).as_bytes());
            let stored_secret_with_redis_prefix =
                format!("{REDIS_USER_CREATION_PENDING_PREFIX}:{}", data_hashed);
            if check_if_exists_cache::<&str, AuthProviderDataCommon>(
                app.clone(),
                &stored_secret_with_redis_prefix,
            )
            .await
            {
                return Err(ServerError::ISErr(Cow::from("failed to create slip")));
            }
            insert_into_cache(
                app.clone(),
                &stored_secret_with_redis_prefix,
                &user_info_common,
                Some(1000),
            )
            .await?;

            RedirectedUser::NewUserCreation {
                slip: stored_secret_with_redis_prefix,
                suggested_username: suggested,
            }
        }
    }))
}

#[instrument]
#[utoipa::path(
    delete,
    path = "/burn_signup_token",
    responses(
    (status = 200, description = "Token Sucessfully burnt"),
    (status = 404, description = "No token exists"),
    (status = 500, description = "Failed")),
    params(
    ("request" = String, query, description = "token")
    )
)]
pub async fn burn_signup_token(
    Extension(state): Extension<Arc<State>>,
    request: Query<String>,
) -> SResult<()> {
    if !check_if_exists_cache::<&str, AuthProviderDataCommon>(state.clone(), &request.0).await {
        return Err(ServerError::NotFound(
            Cow::from("signup in progress"),
            Cow::from(request.0),
        ));
    }
    delet_dis::<AuthProviderDataCommon>(state.clone(), &request.0).await?;
    Ok(())
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Component)]
pub struct PostSignupSent {
    pub id: u64,
    pub login_secret: JWTPair,
}

#[instrument]
#[utoipa::path(
    post,
    path = "/signup",
    request_body = UserSignupRequest,
    responses(
    (status = 200, description = "Thank you %user! But our HRT is in another site! (diyhrt.github.io)", body = UserSignupRequest),
    (status = 400, description = "Signup Request Modified/Invalid"),
    (status = 404, description = "Signup Request Does Not Exist"),
    (status = 500, description = "Failed")),
    params(
    ("request" = String, query, description = "Request Signup Token")
    )
)]
pub async fn signup(
    Extension(state): Extension<Arc<State>>,
    request: Query<String>,
    data: Json<UserSignupRequest>,
) -> SResult<Json<PostSignupSent>> {
    if !(data.username.len() > 30
        || data.profile_picture.len() > 200
        || data.email.len() > 100
        || data.other_data.verify())
    {
        return Err(ServerError::BadRequest(Cow::from("too long!")));
    }

    let oauth_data = read_from_cache::<AuthProviderDataCommon>(state.clone(), &request.0).await?;
    let user_data = data.0;

    let user_id = state.id_generator.user_ids.generate_id();

    if user_by_id(state.clone(), user_id).await.is_ok() {
        return Err(ServerError::ISErr(Cow::from("please retry")));
    }

    let oauth_email = match oauth_data.email.as_ref() {
        Some(e) => e.clone(),
        None => "".to_string(),
    };

    if oauth_data.email.is_none()
        || user_data.email != oauth_email
        || user_by_email(state.clone(), &oauth_email).await?.is_some()
    {
        return Err(ServerError::BadRequest(Cow::from("email")));
    }

    if user_by_username(state.clone(), &user_data.username)
        .await?
        .is_some()
    {
        return Err(ServerError::BadRequest(Cow::from("username")));
    }

    let user_active_model = user::ActiveModel {
        id: ActiveValue::Set(user_id),
        username: ActiveValue::Set(user_data.username),
        email: ActiveValue::Set(oauth_data.email.unwrap()),
        profile_picture: ActiveValue::Set(Some(oauth_data.profile_picture)),
        creation_date: ActiveValue::Set(Utc::now()),
        roles: ActiveValue::Set(Role::NormalUser),
    };

    let user_data_active_model = userdata::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        gender: ActiveValue::Set(user_data.other_data.gender),
        pronouns: ActiveValue::Set(user_data.other_data.pronouns),
        birthday: ActiveValue::Set(user_data.other_data.birthday),
        locale: ActiveValue::Set(user_data.other_data.locale),
    };

    user::Entity::insert(user_active_model)
        .exec(&state.database)
        .await?;
    userdata::Entity::insert(user_data_active_model)
        .exec(&state.database)
        .await?;

    let login_generated = generate_login_token(state.clone(), user_id).await?;

    Ok(Json(PostSignupSent {
        id: user_id,
        login_secret: login_generated,
    }))
}

route! {
    "/redirect" => post(redirect),
    "/burn_signup_token" => delete(burn_signup_token),
    "/signup" => post(signup)
}
