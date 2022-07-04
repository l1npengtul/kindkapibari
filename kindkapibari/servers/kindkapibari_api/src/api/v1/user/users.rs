use crate::{
    access::user::{update_user_data_by_user_id, user_by_id, user_data_by_user_id},
    api::auth::AuthedUser,
    SResult, ServerError, State,
};
use axum::{extract::Path, Extension, Json};
use kindkapibari_core::{auth::Authentication, user_data::UserData};
use kindkapibari_schema::{auth::AuthedUser, SResult};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
pub async fn username(Extension(state): Extension<Arc<State>>, id: Path<u64>) -> SResult<String> {
    let user = user_by_id(state, id.0).await?;
    Ok(user.username)
}

#[instrument]
pub async fn profile_picture(
    Extension(state): Extension<Arc<State>>,
    id: Path<u64>,
) -> SResult<String> {
    let user = user_by_id(state, id.0).await?;
    Ok(user.profile_picture.ok_or(ServerError::NotFound(
        Cow::from("profile picture"),
        Cow::from(id.0.to_string()),
    ))?)
}

#[instrument]
pub async fn account_creation_date(
    Extension(state): Extension<Arc<State>>,
    id: Path<u64>,
) -> SResult<String> {
    let user = user_by_id(state, id.0).await?;
    Ok(user.username)
}

#[instrument]
pub async fn get_user_data(
    Extension(state): Extension<Arc<State>>,
    user: u64,
) -> SResult<Json<UserData>> {
    let userdata = user_data_by_user_id(state.clone(), auth.0.id).await?;
    Ok(Json(userdata))
}

#[instrument]
pub async fn set_user_data(
    Extension(state): Extension<Arc<State>>,
    auth: Authentication<AuthedUser>,
    new_data: Json<UserData>,
) -> SResult<()> {
    update_user_data_by_user_id(state, auth.0.id, new_data.0).await?;
    Ok(())
}
