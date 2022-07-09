use crate::{
    access::{
        login::{refresh_user_login_token, verify_user_login_token},
        oauth_thirdparty::{oauth_login_github, oauth_login_twitter, OAuthAttempt},
    },
    State,
};
use axum::{extract::Query, routing::post, Extension, Json};
use kindkapibari_core::{
    route,
    secret::{JWTPair, SentSecret},
};
use kindkapibari_schema::{
    error::ServerError,
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
    (status = 200, description = "Sucessfully Verified Token", body = Model),
    (status = 400, description = "Invalid Token"),
    (status = 401, description = "Invalid Token"),
    (status = 403, description = "Invalid Token"),
    (status = 404, description = "User Not Found"),
    (status = 500, description = "Failed")),
    params(
    ("token" = String, query, description = "token to verify (access JWT)")
    )
)]
pub async fn verify_login_token(
    Extension(app): Extension<Arc<State>>,
    token: Query<String>,
) -> SResult<Json<Model>> {
    verify_user_login_token(app, token.0).await.map(|x| Json(x))
}

#[instrument]
#[utoipa::path(
    post,
    path = "/refresh_token",
    responses(
    (status = 200, description = "Sucessfully refreshed tokens", body = Model),
    (status = 400, description = "Invalid Token"),
    (status = 401, description = "Invalid Token"),
    (status = 403, description = "Invalid Token"),
    (status = 404, description = "User Not Found"),
    (status = 500, description = "Failed")),
    params(
    ("token" = String, query, description = "token to verify (access JWT)")
    )
)]
pub async fn refresh_token(
    Extension(app): Extension<Arc<State>>,
    access: Query<String>,
    refresh: Query<String>,
) -> SResult<Json<JWTPair>> {
    refresh_user_login_token(app, access.0, refresh.0)
        .await
        .map(|x| Json(x))
}

route! {
    "/login_with_twitter" => post(login_with_twitter),
    "/login_with_github" => post(login_with_github),
    "/verify_login_token" => post(verify_login_token),
    "/refresh_token" => post(refresh_token)
}
