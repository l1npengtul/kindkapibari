use crate::access::auth::oauth::application_by_id;
use crate::{AppData, SResult, ServerError};
use oxide_auth_poem::request::OAuthRequest;
use poem::handler;
use poem::http::Uri;
use poem::web::{Data, Query};
use std::sync::Arc;
use tracing::instrument;

const REDIS_AUTHORIZE_LOGIN_REDIRECT_ID_HEADER: &'static str = "kkb:au_lg_rdr:";

#[instrument]
// #[handler]
pub async fn authorize(
    Data(app): Data<Arc<AppData>>,
    req: &OAuthRequest,
    Query(response_type): Query<String>,
    Query(client_id): Query<String>,
    Query(redirect_uri): Query<Option<Uri>>,
    Query(scopes): Query<Option<String>>,
    Query(state): Query<String>,
) -> SResult<impl Response> {
    if response_type != "code" {
        return Err(ServerError::BadRequest("Must be code"));
    }
    // see if the application exists
    let app_id = application_by_id(app, client_id.parse::<u64>()?).await?;
    // see if user is logged in
}

#[instrument]
// #[handler]
pub async fn token() {}
