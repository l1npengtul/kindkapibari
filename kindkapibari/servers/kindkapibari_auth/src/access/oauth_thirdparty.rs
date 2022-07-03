use kindkapibari_core::{impl_redis, impl_sea_orm};
use kindkapibari_schema::{error::ServerError, SResult};
use oauth2::{
    basic::{BasicClient, BasicTokenResponse},
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use paste::paste;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt::Debug, sync::Arc};
use tracing::instrument;

pub type AResult<T> = Result<T, Box<dyn std::error::Error>>;

trait OAuthProviderBasicInfo {
    fn id_as_str(&self) -> String;
    fn username(&self) -> &str;
    fn handle(&self) -> &str;
    fn profile_picture_url(&self) -> &str;
    fn email(&self) -> &Option<String>;
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Twitter {
    pub twitter_id: u64,
    pub username: String,
    pub handle: String,
    pub profile_picture: String,
    pub email: Option<String>, // Always `None` for now due to twitter API v2
}

impl OAuthProviderBasicInfo for Twitter {
    fn id_as_str(&self) -> String {
        self.twitter_id.to_string()
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

    fn email(&self) -> &Option<String> {
        &None
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Github {
    pub github_id: i64,
    pub username: String,
    pub profile_picture: String,
    pub email: Option<String>,
}

impl OAuthProviderBasicInfo for Github {
    fn id_as_str(&self) -> String {
        self.github_id.to_string()
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

    fn email(&self) -> &Option<String> {
        &self.email
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorizationProviders {
    Twitter(Twitter),
    Github(Github),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthProviderDataCommon {
    pub id: u64,
    pub username: String,
    pub profile_picture: String,
    pub email: Option<String>,
}

impl From<AuthorizationProviders> for AuthProviderDataCommon {
    #[allow(clippy::cast_sign_loss)]
    fn from(authp: AuthorizationProviders) -> Self {
        match authp {
            AuthorizationProviders::Twitter(twt) => Self {
                id: twt.twitter_id,
                username: twt.username,
                profile_picture: twt.profile_picture,
                email: twt.email,
            },
            AuthorizationProviders::Github(ghb) => Self {
                id: ghb.github_id as u64,
                username: ghb.username,
                profile_picture: ghb.profile_picture,
                email: ghb.email,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OAuthAttempt {
    auth_url: String,
    csrf_token: String,
    pkce_verifier: String,
    authorizer: String,
}

impl OAuthAttempt {
    #[must_use]
    pub fn auth_url(&self) -> &str {
        &self.auth_url
    }
    #[must_use]
    pub fn csrf_token(&self) -> &str {
        &self.csrf_token
    }
    #[must_use]
    pub fn pkce_verifier(&self) -> &str {
        &self.pkce_verifier
    }
    #[must_use]
    pub fn authorizer(&self) -> &str {
        &self.authorizer
    }
}

pub fn get_oauth_client(
    authorize_url: String,
    token_url: String,
    redirect_url: String,
    client_id: String,
    client_secret: String,
) -> Result<BasicClient, Box<dyn std::error::Error>> {
    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(authorize_url)?,
        Some(TokenUrl::new(token_url)?),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url)?))
}

macro_rules! oauth_providers {
    ($( [$provider:ident { $($scope:expr),* }] ),*) => {
        $(
            paste! {
                #[instrument]
                pub async fn [<oauth_login_ $provider>](state: Arc<crate::State>, url: &str) -> SResult<OAuthAttempt> {
                    let config = state.config.read().await.clone();
                    let client = get_oauth_client(
                        config.oauth.$provider.authorize_url,
                        config.oauth.$provider.token_url,
                        format!("{url}/redirect"),
                        config.oauth.$provider.client_id,
                        config.oauth.$provider.secret,
                    ).map_err(|x| ServerError::InternalServer(x))?;
                    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256_len(96);

                    let (auth_url, csrf) = client
                        .authorize_url(CsrfToken::new_random)
                        $(
                        .add_scope(Scope::new($scope.to_string()))
                        )*
                        .set_pkce_challenge(pkce_challenge)
                        .url();

                    return Ok(
                        OAuthAttempt {
                            auth_url: auth_url.to_string(),
                            csrf_token: csrf.secret().to_string(),
                            pkce_verifier: pkce_verifier.secret().to_string(),
                            authorizer: stringify!($provider).to_string(),
                        }
                    );
                }
            }
        )*
    };
}

oauth_providers!([twitter {"users.read", "tweet.read"}], [github {"read:user", "user:email"}]);

#[instrument]
pub async fn get_user_data(
    authorizer: &str,
    token: BasicTokenResponse,
) -> SResult<AuthorizationProviders> {
    match authorizer {
        "twitter" => Ok(AuthorizationProviders::Twitter(
            get_twitter_info(token)
                .await
                .map_err(|x| ServerError::InternalServer(x))?,
        )),
        "github" => Ok(AuthorizationProviders::Github(
            get_github_info(token)
                .await
                .map_err(|x| ServerError::InternalServer(x))?,
        )),
        _ => Err(ServerError::BadRequest(Cow::from("bad authorizer"))),
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
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .json::<TwitterUser>()
        .await?;

    Ok(Twitter {
        twitter_id: user.data.id.parse()?,
        username: user.data.name,
        handle: user.data.username,
        profile_picture: user.data.profile_image_url,
        email: None,
    })
}

#[instrument]
async fn get_github_info(token: BasicTokenResponse) -> AResult<Github> {
    #[derive(Default, Serialize, Deserialize)]
    #[serde(default)]
    struct GithubUser {
        id: i64,
        login: String,
        avatar_url: String,
        email: Option<String>,
    }

    let user = Client::new()
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
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

impl_sea_orm!(
    Twitter,
    Github,
    AuthorizationProviders,
    AuthProviderDataCommon,
    OAuthAttempt
);
impl_redis!(
    Twitter,
    Github,
    AuthorizationProviders,
    AuthProviderDataCommon,
    OAuthAttempt
);
