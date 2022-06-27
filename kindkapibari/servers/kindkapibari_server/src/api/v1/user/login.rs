use crate::access::auth::login::burn_login_token;
use crate::{AppData, SResult, ServerError};
use axum::extract::Query;
use axum::Extension;
use kindkapibari_core::secret::SentSecret;
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn login_post(
    Extension(_): Extension<Arc<AppData>>,
    _: String,
    _: String,
) -> SResult<SentSecret> {
    Err(ServerError::NotImplemented)
}

#[derive(Deserialize)]
struct Logout {
    pub user: u64,
    pub token: String,
}

#[instrument]
pub async fn logout(Extension(state): Extension<Arc<AppData>>, args: Query<Logout>) -> SResult<()> {
    burn_login_token(
        state.clone(),
        args.user,
        SentSecret::from_str_token(&args.token)
            .ok_or(ServerError::BadRequest(Cow::from("parse failure")))?,
    )
    .await
}
