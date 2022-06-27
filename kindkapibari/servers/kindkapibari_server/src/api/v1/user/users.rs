use crate::access::user::{
    get_sobers_by_user_id, update_user_data_by_user_id, user_data_by_user_id,
};
use crate::api::auth::AuthedUser;
use crate::{AppData, SResult};
use axum::{Extension, Json};
use kindkapibari_core::auth::Authentication;
use kindkapibari_core::sober::Sobers;
use kindkapibari_core::user_data::UserData;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn get_user_data(
    Extension(state): Extension<Arc<AppData>>,
    auth: Authentication<AuthedUser>,
) -> SResult<Json<UserData>> {
    let userdata = user_data_by_user_id(state.clone(), auth.0.id).await?;
    Ok(Json(userdata))
}

#[instrument]
pub async fn set_user_data(
    Extension(state): Extension<Arc<AppData>>,
    auth: Authentication<AuthedUser>,
    new_data: Json<UserData>,
) -> SResult<()> {
    update_user_data_by_user_id(state, auth.0.id, new_data.0).await?;
    Ok(())
}

#[instrument]
pub async fn get_user_sobers(
    Extension(state): Extension<Arc<AppData>>,
    auth: Authentication<AuthedUser>,
) -> SResult<Json<Sobers>> {
    let sobers = get_sobers_by_user_id(state, auth.0.id).await?;
    Ok(Json(sobers))
}
