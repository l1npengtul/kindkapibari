use crate::{AppData, SResult};
use poem::handler;
use poem::web::Data;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
#[handler]
pub async fn login_with_twitter(Data(app): Data<Arc<AppData>>) -> SResult<impl Response> {}

#[instrument]
#[handler]
pub async fn login_with_github(Data(app): Data<Arc<AppData>>) -> SResult<impl Response> {}

#[instrument]
#[handler]
pub async fn redirect(Data(app): Data<Arc<AppData>>, redirect_id: u64) {}
