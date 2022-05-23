use crate::access::auth::login::verify_login_token;
use crate::access::auth::oauth::{application_by_id, verify_access_token};
use crate::access::{insert_into_cache, TOKEN_SEPERATOR};
use crate::{AppData, KKBScope, SResult, ServerError, THIS_SITE_URL};
use chrono::{TimeZone, Utc};
use kindkapibari_core::impl_redis;
use kindkapibari_core::secret::{decode_gotten_secret, DecodedSecret};
use kindkapibari_core::snowflake::SnowflakeIdGenerator;
use once_cell::sync::Lazy;
use oxide_auth_poem::request::OAuthRequest;
use oxide_auth_poem::response::OAuthResponse;
use poem::http::uri::Scheme;
use poem::http::Uri;
use poem::web::{Data, Query, Redirect};
use poem::{handler, IntoResponse, Request};
use poem_openapi::auth::Bearer;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::sync::Arc;
use tracing::instrument;

const REDIS_AUTHORIZE_LOGIN_REDIRECT_ID_HEADER: &'static str = "kkb:au_lg_rdr:";

static ID_GENERATOR: Lazy<Arc<SnowflakeIdGenerator>> = Lazy::new(|| {
    Arc::new(SnowflakeIdGenerator::new(
        Utc.timestamp_millis(16502056_420_69), // nice
    ))
});

#[derive(Serialize, Deserialize)]
struct AuthorizeRequest {
    client_id: String,
    client_secret: String,
    redirect_url: Option<String>,
    scopes: Vec<KKBScope>,
    state: String,
}

impl_redis!(AuthorizeRequest);

#[instrument]
// #[handler]
pub async fn authorize(
    Data(state): Data<Arc<AppData>>,
    request: &OAuthRequest,
) -> SResult<impl IntoResponse> {
    // if response_type != "code" {
    //     return Err(ServerError::BadRequest("Must be code"));
    // }
    // let config = app.config.read().await;
    //
    // // see if the application exists
    // let app_id = application_by_id(app.clone(), client_id.parse::<u64>()?).await?;
    // // see if user is logged in
    // let token = match bearer {
    //     Ok(b) => decode_gotten_secret(
    //         base64::decode(b.token)?.as_str()?,
    //         TOKEN_SEPERATOR,
    //         config.signing_key.as_bytes(),
    //     )?,
    //     Err(_) => return redirect_to_login_with_redirect(request.uri()),
    // };
    //
    // let user = match verify_login_token(app.clone(), token).await {
    //     Ok(u) => u,
    //     Err(_) => return redirect_to_login_with_redirect(request.uri()),
    // };
    // // present
}

#[instrument]
// #[handler]
pub async fn authorize_consent(
    Data(state): Data<Arc<AppData>>,
    request: &OAuthRequest,
    allow: poem::Result<Query<bool>>,
) -> SResult<OAuthResponse> {
}

#[instrument]
// #[handler]
pub async fn redirecting(Data(app): Data<Arc<AppData>>, redirect_id: u128) {}

#[instrument]
// #[handler]
pub async fn token() {}
