use crate::{access::insert_without_timeout, AResult, AppData, Config, EResult, SResult};
use oauth2::basic::BasicTokenResponse;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};
use poem::{
    error::{InternalServerError, NotFound, Unauthorized},
    web::{Data, Query, Redirect},
    IntoResponse,
};
use redis::AsyncCommands;
use std::sync::Arc;
use tracing::instrument;

fn get_client(config: Config) -> SResult<BasicClient> {
    let auth_url = AuthUrl::new(config.oauth.authorize_url)?;
    let token_url = TokenUrl::new(config.oauth.token_url)?;
    let redirect_url = RedirectUrl::new(&config.host_url + "/redirect")?;
    Ok(BasicClient::new(
        ClientId::new(config.oauth.client_id),
        Some(ClientSecret::new(config.oauth.client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(redirect_url))
}
// #[handler]
pub async fn login(state: Data<Arc<AppData>>) -> EResult<impl IntoResponse> {
    let config = *state.config.read().await;
    let client = get_client(config).map_err(InternalServerError)?;

    let (challenge_pkce, verifier_pkce) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random_len(64))
        .set_pkce_challenge(challenge_pkce)
        .add_scope(Scope::new("public_read".to_string()))
        .add_scope(Scope::new("email_read".to_string()))
        .url();

    insert_without_timeout(state.0, csrf_token.secret(), verifier_pkce.secret())
        .await
        .map_err(InternalServerError)?;
    Ok(Redirect::see_other(auth_url))
}

// #[handler]
pub async fn redirect(
    app_state: Data<Arc<AppData>>,
    Query(state): Query<String>,
    Query(code): Query<String>,
) -> EResult<impl IntoResponse> {
    let config = *state.config.read().await;
    let verifier_pkce = app_state
        .redis
        .get::<String, String>(state)
        .await
        .map_err(NotFound)?;
    let client = get_client(config).map_err(InternalServerError)?;
    let token = client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(verifier_pkce))
        .request_async(async_http_client)
        .await
        .map_err(Unauthorized)?;

    // TODO: commit token to storage
    //

    Ok("Welcome to CoconutPak Island, prepare to e x p e r i e n c e sexual allegories for exploitiative systems. (pemng chan alreaddy has taken all the coconut paks)")
}

#[instrument]
pub async fn token_to_user(state: Arc<AppData>, token: BasicTokenResponse) -> SResult<u64> {}
