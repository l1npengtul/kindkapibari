use crate::{
    access::{
        sobers::{add_sober, delete_sober, get_sobers, reset_sober, update_sober},
        user::user_by_id,
    },
    api::auth::UserAuthMdl,
    State,
};
use axum::{
    extract::Path,
    routing::{delete, get, patch, post},
    Extension, Json,
};
use kindkapibari_core::{
    auth::Authentication,
    roles::Role,
    route,
    sober::{Sober, Sobers},
};
use kindkapibari_schema::{error::ServerError, SResult};
use std::sync::Arc;
use tracing::instrument;

#[instrument]
#[utoipa::path(
    post,
    path = "/users/sobers/{id}",
    responses(
    (status = 200, description = "Sobers", body = Sobers),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist."),
    (status = 500, description = "Failed")),
    params(
    ("id" = u64, path, description = "User ID")
    ),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn get_user_sobers(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Path(user_id): Path<u64>,
) -> SResult<Json<Sobers>> {
    let mut user = user;
    if user.roles <= Role::Server || user.id != user_id {
        return Err(ServerError::Unauthorized);
    }
    user = user_by_id(state.clone(), user_id).await?.into();

    let sobers = get_sobers(state, user.into()).await?;
    Ok(Json(sobers))
}

#[instrument]
#[utoipa::path(
    patch,
    path = "/users/reset/{sober_id}",
    responses(
    (status = 200, description = "Sucessfully reset sober time"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Sober does not exist"),
    (status = 500, description = "Failed")),
    params(
    ("user_id" = u64, path, description = "User ID"),
    ("sober_id" = u64, path, description = "Sober ID"),
    ),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn patch_user_sober_reset_time(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Path(sober_id): Path<u64>,
) -> SResult<()> {
    reset_sober(state, sober_id, user.id).await?;
    Ok(())
}

#[instrument]
#[utoipa::path(
    patch,
    path = "/users/update_sober",
    request_body = Sober,
    responses(
    (status = 200, description = "Sucessfully Updated Sober"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Sober does not exist"),
    (status = 500, description = "Failed")),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn patch_update_sober(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Json(sober): Json<Sober>,
) -> SResult<()> {
    update_sober(state, sober.id, sober.name, user.into()).await?;
    Ok(())
}

#[instrument]
#[utoipa::path(
    post,
    path = "/users/add_sober",
    request_body = Sober,
    responses(
    (status = 200, description = "Sucessfully Updated Sober, Sober ID", body = u64),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Sober does not exist"),
    (status = 500, description = "Failed")),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn post_add_sober(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Json(sober): Json<Sober>,
) -> SResult<Json<u64>> {
    let id = add_sober(state, user.into(), sober).await?;
    Ok(Json(id))
}

#[instrument]
#[utoipa::path(
    delete,
    path = "/users/delete_sober/{id}",
    request_body = Sober,
    responses(
    (status = 200, description = "Sucessfully Updated Sober"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Sober does not exist"),
    (status = 500, description = "Failed")),
    params(
    ("id" = u64, path, description = "Sober ID")
    ),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn delete_user_sober(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    id: Path<u64>,
) -> SResult<()> {
    delete_sober(state, user.id, id.0).await?;
    Ok(())
}

route! {
    "/sobers/:id" => get(get_user_sobers),
    "/reset/:id" => patch(patch_user_sober_reset_time),
    "/update_sober" => patch(patch_update_sober),
    "/add_sober" => post(post_add_sober),
    "/delete_sober/:id" => delete(delete_user_sober)
}
