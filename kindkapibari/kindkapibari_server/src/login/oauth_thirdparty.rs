use crate::access::auth::oauth_thirdparty::{
    get_oauth_client, get_user_data, AuthorizationProviders, OAuthAttempt,
};
use crate::access::check_if_exists_cache;
use crate::{
    access::{
        auth::oauth_thirdparty::{oauth_login_github, oauth_login_twitter},
        insert_into_cache,
    },
    AResult, AppData, SResult, ServerError, THIS_SITE_URL,
};
use chrono::Utc;
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, PkceCodeVerifier};
use once_cell::sync::Lazy;
use poem::web::{Json, Query};
use poem::{
    handler,
    web::{Data, Redirect},
};
use redis::{AsyncCommands, RedisResult};
use std::{borrow::Cow, sync::Arc};
use tokio::sync::Mutex;
use tracing::instrument;
use tracing_subscriber::fmt::time;

static AUTO_RESEEDING_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

#[instrument]
async fn generate_redirect_id(state: Arc<AppData>) -> String {
    let salt = app
        .clone()
        .id_generator
        .redirect_ids
        .generate_id()
        .to_be_bytes()
        .as_bytes();
    let rng_gen: [u8; 64] = AUTO_RESEEDING_RNG.lock().await.generate_bytes();
    base64::encode(blake3::hash(&[rng_gen, salt].concat()))
}

#[instrument]
#[handler]
pub async fn login_with_twitter(Data(app): Data<Arc<AppData>>) -> SResult<Redirect> {
    let oauth = oauth_login_twitter(app.clone(), THIS_SITE_URL).await?;
    let redirect = Redirect::see_other(oauth.auth_url());

    if check_if_exists_cache(app.clone(), &redirect_id) {
        return Err(ServerError::ISErr(Cow::Borrowed(
            "ID already exists, please try again!",
        )));
    }
    insert_into_cache(app, redirect_id, oauth, Some(1000)).await?;
    Ok(redirect)
}

#[instrument]
#[handler]
pub async fn login_with_github(Data(app): Data<Arc<AppData>>) -> SResult<Redirect> {
    let oauth = oauth_login_github(app.clone(), THIS_SITE_URL).await?;
    let redirect = Redirect::see_other(oauth.auth_url());

    if check_if_exists_cache(app.clone(), &redirect_id) {
        return Err(ServerError::ISErr(Cow::Borrowed(
            "ID already exists, please try again!",
        )));
    }
    insert_into_cache(app, redirect_id, oauth, Some(1000)).await?;
    Ok(redirect)
}

#[instrument]
#[handler]
pub async fn redirect(
    Data(app): Data<Arc<AppData>>,
    state: Query<String>,
    code: Query<String>,
) -> SResult<Json<AuthorizationProviders>> {
    let oauth_attempt = app.redis.get::<&String, OAuthAttempt>(&redirect_id).await?;
    let config = *app.config.read().await;
    if state != oauth_attempt.pkce_verifier() {
        return Err(ServerError::BadRequest(Cow::Borrowed("Bad State")));
    }

    let client_rebuild = match oauth_attempt.authorizer() {
        "twitter" => get_oauth_client(
            config.oauth.twitter.authorize_url,
            config.oauth.twitter.token_url,
            format!("{THIS_SITE_URL}/redirect"),
            config.oauth.twitter.client_id,
            config.oauth.twitter.secret,
        )?,
        "github" => get_oauth_client(
            config.oauth.github.authorize_url,
            config.oauth.github.token_url,
            format!("{THIS_SITE_URL}/redirect"),
            config.oauth.github.client_id,
            config.oauth.github.secret,
        )?,
        _ => return Err(ServerError::BadRequest(Cow::Borrowed("Bad Authorizer"))),
    };

    let token_result = client_rebuild
        .exchange_code(AuthorizationCode::new(code.0))
        .set_pkce_verifier(PkceCodeVerifier::new(
            oauth_attempt.pkce_verifier().to_string(),
        ))
        .request_async(async_http_client)
        .await
        .map_err(|why| ServerError::InternalServer(why))?;

    let user_info = get_user_data(oauth_attempt.authorizer(), token_result).await?;
    Ok(Json(user_info))
}
