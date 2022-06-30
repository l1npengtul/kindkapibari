use crate::{SResult, ServerError, State};
use axum::{extract::Query, Extension, Json};
use kindkapibari_core::{sober::Sober, sober::Sobers};
use kindkapibari_schema::{
    access::user::{
        add_sober_by_user, delete_sober_name_by_user_id, get_sobers_by_user_id,
        reset_sober_by_name_and_user_id, update_sober_name_by_user_id,
    },
    auth::AuthedUser,
};
use serde::Deserialize;
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
pub async fn get_user_sobers(
    Extension(state): Extension<Arc<State>>,
    id: u64,
) -> SResult<Json<Sobers>> {
    let sobers = get_sobers_by_user_id(state, id).await?;
    Ok(Json(sobers))
}

#[derive(Deserialize)]
struct UserSoberReq {
    pub name: String,
}

#[instrument]
pub async fn patch_user_sober_reset_time(
    Extension(state): Extension<Arc<State>>,
    id: u64,
    name: Query<UserSoberReq>,
) -> SResult<()> {
    reset_sober_by_name_and_user_id(state, name.0.name, id).await?;
    Ok(())
}

#[derive(Deserialize)]
struct UpdateSoberQueryReq {
    pub old_name: String,
    pub new_name: String,
}

#[instrument]
pub async fn update_sober(
    Extension(state): Extension<Arc<State>>,
    id: u64,
    name: Query<UpdateSoberQueryReq>,
) -> SResult<()> {
    update_sober_name_by_user_id(state, name.0.old_name, name.0.new_name, id).await?;
    Ok(())
}

#[instrument]
pub async fn add_sober(
    Extension(state): Extension<Arc<State>>,
    id: u64,
    sober: Json<Sober>,
) -> SResult<()> {
    // check sober name len
    if sober.name.len() > 100 {
        return Err(ServerError::BadRequest(Cow::from("too long name!")));
    }
    add_sober_by_user(state, id, sober.0).await?;
    Ok(())
}

#[instrument]
pub async fn delete_sober(
    Extension(state): Extension<Arc<State>>,
    id: u64,
    sober: Query<UserSoberReq>,
) -> SResult<()> {
    delete_sober_name_by_user_id(state, id, sober.0.name).await?;
    Ok(())
}
