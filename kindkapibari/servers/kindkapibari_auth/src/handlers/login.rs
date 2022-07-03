use crate::{
    access::{
        login::verify_user_login_token,
        oauth_thirdparty::{oauth_login_github, oauth_login_twitter, OAuthAttempt},
    },
    State,
};
use axum::{extract::Query, Extension, Json};
use kindkapibari_core::secret::SentSecret;
use kindkapibari_schema::{
    error::ServerError,
    opt_to_sr,
    redis::{check_if_exists_cache, insert_into_cache},
    schema::users::user::Model,
    SResult,
};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

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
    let pkce = oauth.pkce_verifier().to_string();

    if check_if_exists_cache::<&str, OAuthAttempt>(app.clone(), oauth.pkce_verifier()).await {
        return Err(ServerError::ISErr(Cow::Borrowed(
            "ID already exists, please try again!",
        )));
    }
    insert_into_cache(app, pkce, oauth, Some(1000)).await?;
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
    let oauth = oauth_login_github(app.clone(), app.config.read().await.host_url.as_str()).await?;
    let redirect = oauth.auth_url().to_string();
    let pkce = oauth.pkce_verifier().to_string();

    if check_if_exists_cache::<&str, OAuthAttempt>(app.clone(), oauth.pkce_verifier()).await {
        return Err(ServerError::ISErr(Cow::Borrowed(
            "ID already exists, please try again!",
        )));
    }
    insert_into_cache(app, pkce, oauth, Some(1000)).await?;
    Ok(redirect)
}

#[instrument]
#[utoipa::path(
    post,
    path = "/verify_login_token",
    responses(
    (status = 200, description = "Github Url Sucessfully Generated", body = Model),
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
) -> SResult<Json<Model>> {
    let user = verify_user_login_token(
        app.clone(),
        SentSecret::from_str_token(&token.0)
            .ok_or_else(|| ServerError::BadRequest(Cow::from("Invalid Token")))?,
    )
    .await?;
    Ok(Json(opt_to_sr!(user, "_", "not found")))
}
