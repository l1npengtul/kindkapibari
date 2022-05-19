use crate::{AppData, SResult, ServerError};
use oxide_auth_poem::request::OAuthRequest;
use poem::handler;
use poem::http::Uri;
use poem::web::{Data, Query};
use std::sync::Arc;
use tracing::instrument;

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
    //
}

#[instrument]
// #[handler]
pub async fn token() {}
