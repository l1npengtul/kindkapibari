use crate::access::login::{generate_login_token, verify_user_login_token};
use crate::access::oauth_thirdparty::{
    get_oauth_client, get_user_data, oauth_login_github, oauth_login_twitter,
    AuthProviderDataCommon, OAuthAttempt,
};
use crate::State;
use axum::{extract::Query, Extension, Json};
use chrono::Utc;
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use kindkapibari_core::roles::Roles;
use kindkapibari_core::secret::SentSecret;
use kindkapibari_core::user_data::{PostSignupSent, UserSignupRequest};
use kindkapibari_schema::access::user::{
    detect_user_already_exists, user_by_email, user_by_id, user_by_username,
};
use kindkapibari_schema::access::{
    check_if_exists_cache, delet_dis, insert_into_cache, read_from_cache,
};
use kindkapibari_schema::error::ServerError;
use kindkapibari_schema::schema::users::{user, userdata};
use kindkapibari_schema::SResult;
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier};
use once_cell::sync::Lazy;
use redis::AsyncCommands;
use sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, clone, sync::Arc};
use tokio::sync::Mutex;
use tracing::instrument;
use utoipa::Component;

static AUTO_RESEEDING_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

// u c pp
// i c pp
// we all c pp
// pee with friends :) vs pee alone :C
pub const REDIS_USER_CREATION_PENDING_PREFIX: &'static str = "ucpp";

#[instrument]
#[utoipa::path(
    post,
    path = "/login_with_twitter",
    responses(
    (status = 200, description = "Twitter Url Sucessfully Generated", body = String),
    (status = 500, description = "Failed")
))]
pub async fn login_with_twitter(Extension(app): Extension<Arc<State>>) -> SResult<String> {
    let oauth = oauth_login_twitter(app.clone(), app.config.read().await.host_url.as_str()).await?;
    let redirect = oauth.auth_url().to_string();

    if check_if_exists_cache(app.clone(), oauth.pkce_verifier()) {
        return Err(ServerError::ISErr(Cow::Borrowed(
            "ID already exists, please try again!",
        )));
    }
    insert_into_cache(app, oauth.pkce_verifier(), oauth, Some(1000)).await?;
    Ok(redirect)
}

#[instrument]
#[utoipa::path(
    post,
    path = "/login_with_github",
    responses(
    (status = 200, description = "Github Url Sucessfully Generated", body = String),
    (status = 500, description = "Failed")
))]
pub async fn login_with_github(Extension(app): Extension<Arc<State>>) -> SResult<String> {
    let oauth = oauth_login_github(app.clone(), THIS_SITE_URL).await?;
    let redirect = oauth.auth_url().to_string();

    if check_if_exists_cache(app.clone(), &redirect_id) {
        return Err(ServerError::ISErr(Cow::Borrowed(
            "ID already exists, please try again!",
        )));
    }
    insert_into_cache(app, redirect_id, oauth, Some(1000)).await?;
    Ok(redirect)
}

#[instrument]
#[utoipa::path(
    post,
    path = "/verify_login_token",
    responses(
    (status = 200, description = "Github Url Sucessfully Generated", body = json),
    (status = 400, description = "Invalid Token"),
    (status = 404, description = "User Not Found"),
    (status = 500, description = "Failed")),
    params(
    ("token" = String, query, description = "token to verify")
    )
)]
pub async fn verify_login_token(
    Extension(app): Extension<Arc<State>>,
    token: Query<String>,
) -> SResult<Json<Option<user::Model>>> {
    let user = verify_user_login_token(
        app.clone(),
        SentSecret::from_str_token(token.0)
            .ok_or(ServerError::BadRequest(Cow::from("Invalid Token")))?,
    )
    .await?;
    Ok(Json(user))
}
