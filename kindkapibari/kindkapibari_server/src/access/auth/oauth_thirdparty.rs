use crate::{AppData, SResult};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, TokenUrl};
use std::sync::Arc;
use tracing::instrument;

pub struct Twitter {
    pub twitter_id: u64,
    pub username: String,
    pub handle: String,
    pub profile_picture: String,
}

pub struct Github {
    pub github_id: u64,
    pub username: u64,
    pub profile_picture: String,
    pub email: Option<String>,
}

pub struct Reddit {
    pub reddit_id: String,
    pub profile_picture: Option<String>,
    pub email: Option<String>,
}

pub enum AuthorizationProviders {
    Twitter,
    Github,
    Reddit,
}

#[instrument]
pub async fn oauth_login_twitter(state: Arc<AppData>) -> SResult<String> {
    let config = *state.config.read().await;
    let client = BasicClient::new(
        ClientId::new(config.oauth.twitter.client_id),
        Some(ClientSecret::new(config.oauth.twitter.secret)),
        AuthUrl::new(config.oauth.authorize_url)?,
        Some(TokenUrl::new(config.oauth.token_url)?),
    )
    .set_redirect_uri(RedirectUrl::new(config.oauth.redirect_url)?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf) = client
        .authorize_url(CsrfToken::new_random())
        .add_scope("tweet.read".to_string())
        .add_scope("users.read".to_string())
}
