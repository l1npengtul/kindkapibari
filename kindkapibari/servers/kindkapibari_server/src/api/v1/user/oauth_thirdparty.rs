use crate::access::auth::login::generate_login_token;
use crate::access::auth::oauth_thirdparty::{AuthProviderDataCommon, AuthorizationProviders};
use crate::access::user::user_by_username;
use crate::{
    access::{
        auth::oauth_thirdparty::{
            get_oauth_client, get_user_data, oauth_login_github, oauth_login_twitter, OAuthAttempt,
        },
        check_if_exists_cache, insert_into_cache,
        user::detect_user_already_exists,
    },
    AppData, SResult, ServerError, THIS_SITE_URL,
};
use axum::{extract::Query, response::Redirect, Extension, Json};
use kindkapibari_core::reseedingrng::AutoReseedingRng;
use kindkapibari_core::secret::{GeneratedToken, SentSecret};
use oauth2::{reqwest::async_http_client, AuthorizationCode, PkceCodeVerifier};
use once_cell::sync::Lazy;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, sync::Arc};
use tokio::sync::Mutex;
use tracing::instrument;

static AUTO_RESEEDING_RNG: Lazy<Arc<Mutex<AutoReseedingRng<65535>>>> =
    Lazy::new(|| Arc::new(Mutex::new(AutoReseedingRng::new())));

// u c pp
// i c pp
// we all c pp
// pee with friends :) vs pee alone :C
pub const REDIS_USER_CREATION_PENDING_PREFIX: &'static str = "ucpp";

#[instrument]
async fn generate_redirect_id(state: Arc<AppData>) -> String {
    let salt = state
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
pub async fn login_with_twitter(Extension(app): Extension<Arc<AppData>>) -> SResult<Redirect> {
    let oauth = oauth_login_twitter(app.clone(), THIS_SITE_URL).await?;
    let redirect = Redirect::to(oauth.auth_url());

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
pub async fn login_with_github(Extension(app): Extension<Arc<AppData>>) -> SResult<Redirect> {
    let oauth = oauth_login_github(app.clone(), THIS_SITE_URL).await?;
    let redirect = Redirect::to(oauth.auth_url());

    if check_if_exists_cache(app.clone(), &redirect_id) {
        return Err(ServerError::ISErr(Cow::Borrowed(
            "ID already exists, please try again!",
        )));
    }
    insert_into_cache(app, redirect_id, oauth, Some(1000)).await?;
    Ok(redirect)
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum RedirectedUser {
    AlreadyExists(SentSecret),
    NewUserCreation {
        slip: String,
        suggested_username: Option<String>,
    },
}

#[derive(Deserialize)]
pub struct StateAndCode {
    pub state: String,
    pub code: String,
}

#[instrument]
#[handler]
pub async fn redirect(
    Extension(app): Extension<Arc<AppData>>,
    state_and_code: Query<StateAndCode>,
) -> SResult<Json<RedirectedUser>> {
    let oauth_attempt = app.redis.get::<&String, OAuthAttempt>(&redirect_id).await?;
    let config = *app.config.read().await;
    if state_and_code.state != oauth_attempt.pkce_verifier() {
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
        .exchange_code(AuthorizationCode::new(state_and_code.code))
        .set_pkce_verifier(PkceCodeVerifier::new(
            oauth_attempt.pkce_verifier().to_string(),
        ))
        .request_async(async_http_client)
        .await
        .map_err(|why| ServerError::InternalServer(why))?;

    let user_info = get_user_data(oauth_attempt.authorizer(), token_result).await?;
    let maybe_existing_user = detect_user_already_exists(app.clone(), user_info).await?;
    let user_info_common: AuthProviderDataCommon = user_info.into();
    Ok(Json(match maybe_existing_user {
        Some(existing) => {
            RedirectedUser::AlreadyExists(generate_login_token(app.clone(), existing).await?)
        }
        None => {
            // in this case we create a "slip" that the user can trade for not making this request again
            if user_info_common.email.is_none() {
                return Err(ServerError::BadRequest(Cow::from("You need an email!")));
            }
            let user_info_num = format!(
                "{}{}{}{}",
                user_info_common.id,
                &user_info_common.email.unwrap_or_default(),
                &user_info_common.profile_picture,
                &user_info_common.username
            );
            // check if username exists
            let suggested = if !user_by_username(app.clone(), &user_info_num)
                .await?
                .is_none()
            {
                Some(user_info_common.username)
            } else {
                None
            };
            // we dont really need this to be cryptographic
            // by the time this is reversed it will already be useless
            let data_hashed = base64::encode(blake3::hash(user_info_num.as_bytes()).as_bytes());
            let stored_secret_with_redis_prefix =
                format!("{REDIS_USER_CREATION_PENDING_PREFIX}:{}", data_hashed);
            if !check_if_exists_cache(app.clone(), &stored_secret_with_redis_prefix).await {
                insert_into_cache(app.clone(), &data_hashed, &user_info_common, Some(1000)).await?;
            } else {
                return Err(ServerError::ISErr(Cow::from("failed to create slip")));
            }

            RedirectedUser::NewUserCreation {
                slip: data_hashed,
                suggested_username: suggested,
            }
        }
    }))
}
