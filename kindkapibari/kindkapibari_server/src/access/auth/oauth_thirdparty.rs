use crate::{access::insert_into_cache, AResult, AppData, SResult};
use color_eyre::Report;
use kindkapibari_core::{impl_redis, impl_sea_orm};
use oauth2::{
    basic::{BasicClient, BasicTokenResponse},
    url::Url,
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use paste::paste;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Arc};
use tracing::instrument;

trait OAuthProviderBasicInfo {
    fn id_as_str(&self) -> &str;
    fn username(&self) -> &str;
    fn handle(&self) -> &str;
    fn profile_picture_url(&self) -> Option<&str>;
    fn email(&self) -> Option<&str>;
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Twitter {
    pub twitter_id: String,
    pub username: String,
    pub handle: String,
    pub profile_picture: String,
    pub email: Option<String>, // Always `None` for now due to twitter API v2
}

impl OAuthProviderBasicInfo for Twitter {
    fn id_as_str(&self) -> &str {
        self.twitter_id.to_string().as_str()
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn handle(&self) -> &str {
        &self.handle
    }

    fn profile_picture_url(&self) -> &str {
        self.profile_picture.as_ref()
    }

    fn email(&self) -> Option<&str> {
        None
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Github {
    pub github_id: i64,
    pub username: String,
    pub profile_picture: String,
    pub email: Option<String>,
}

impl OAuthProviderBasicInfo for Github {
    fn id_as_str(&self) -> &str {
        self.github_id.to_string().as_str()
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn handle(&self) -> &str {
        self.username()
    }

    fn profile_picture_url(&self) -> &str {
        self.profile_picture.as_ref()
    }

    fn email(&self) -> Option<&str> {
        self.email.as_ref().into()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AuthorizationProviders {
    Twitter(Twitter),
    Github(Github),
}

impl_sea_orm!(Twitter, Github, AuthorizationProviders);
impl_redis!(Twitter, Github, AuthorizationProviders);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct OAuthAttempt<'a> {
    pkce_verifier: &'a String,
    authorizer: &'static str,
}

fn get_oauth_client(
    authorize_url: String,
    token_url: String,
    redirect_url: String,
    client_id: String,
    client_secret: String,
) -> BasicClient {
    return BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(authorize_url)?,
        Some(TokenUrl::new(token_url)?),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?);
}

macro_rules! oauth_providers {
    ($( [$provider:ident { $($scope:expr),* }] ),*) => {
        $(
            paste! {
                #[instrument]
                pub async fn [<oauth_login_ $provider>](state: Arc<AppData>) -> SResult<Url> {
                    let config = *state.config.read().await;
                    let client = get_oauth_client(
                        config.oauth.$provider.authorize_url,
                        config.oauth.$provider.token_url,
                        config.oauth.redirect_url,
                        config.oauth.$provider.client_id,
                        config.oauth.$provider.secret,
                    );
                    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256_len(96);

                    let (auth_url, csrf) = client
                        .authorize_url(CsrfToken::new_random())
                        $(
                        .add_scope(Scope::new($scope.to_string()))
                        )*
                        .set_pkce_challenge(pkce_challenge)
                        .url();

                    // commit to RAM
                    insert_into_cache(
                        state,
                        csrf.secret(),
                        OAuthAttempt {
                            pkce_verifier: pkce_verifier.secret(),
                            authorizer: stringify!($provider),
                        },
                        Some(120),
                    )
                    .await?;

                    return auth_url;
                }
            }
        )*
    };
}

oauth_providers!([twitter {"users.read", "tweet.read"}], [github {"read:user", "user:email"}]);

#[instrument]
pub async fn get_user_data(
    authorizer: &'static str,
    token: BasicTokenResponse,
) -> AResult<AuthorizationProviders> {
    match authorizer {
        "twitter" => Ok(AuthorizationProviders::Twitter(
            get_twitter_info(token).await?,
        )),
        "github" => Ok(AuthorizationProviders::Github(
            get_github_info(token).await?,
        )),
        _ => Err(Report::msg("must be twitter or github")),
    }
}

#[instrument]
async fn get_twitter_info(token: BasicTokenResponse) -> AResult<Twitter> {
    #[derive(Serialize, Deserialize)]
    struct TwitterUserData {
        profile_image_url: String,
        id: String,
        username: String,
        name: String,
    }

    #[derive(Serialize, Deserialize)]
    struct TwitterUser {
        data: TwitterUserData,
    }

    let user = Client::new()
        .get("https://api.twitter.com/2/users/me?user.fields=id,name,username,profile_image_url")
        .bearer_auth(token.access_token())
        .send()
        .await?
        .json::<TwitterUser>()
        .await?;

    Ok(Twitter {
        twitter_id: user.data.id,
        username: user.data.name,
        handle: user.data.username,
        profile_picture: user.data.profile_image_url,
        email: None,
    })
}

#[instrument]
async fn get_github_info(token: BasicTokenResponse) -> AResult<Github> {
    #[derive(Serialize, Deserialize)]
    #[serde(default)]
    struct GithubUser {
        id: i64,
        login: String,
        avatar_url: String,
        email: Option<String>,
    }

    let user = Client::new()
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token())
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?
        .json::<GithubUser>()
        .await?;

    Ok(Github {
        github_id: user.id,
        username: user.login,
        profile_picture: user.avatar_url,
        email: user.email,
    })
}
