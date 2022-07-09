use crate::{
    access::user::{
        update_user_data_by_user_id, user_by_id, user_by_username, user_data_by_user_id,
    },
    api::auth::UserAuthMdl,
    ServerError, State,
};
use axum::{
    extract::Path,
    routing::{get, patch},
    Extension, Json,
};
use kindkapibari_core::{auth::Authentication, route, user_data::UserData};
use kindkapibari_schema::SResult;
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
#[utoipa::path(
    get,
    path = "/users/username/{id}",
    responses(
    (status = 200, description = "Username", body = String),
    (status = 404, description = "User does not exist."),
    (status = 500, description = "Failed")),
    params(
    ("id" = u64, path, description = "User ID")
    ),
    security(
    ()
    )
)]
pub async fn username(Extension(state): Extension<Arc<State>>, id: Path<u64>) -> SResult<String> {
    let user = user_by_id(state, id.0).await?;
    Ok(user.username)
}

#[instrument]
#[utoipa::path(
    get,
    path = "/users/user_id/{username}",
    responses(
    (status = 200, description = "User ID", body = u64),
    (status = 404, description = "User does not exist."),
    (status = 500, description = "Failed")),
    params(
    ("name" = String, path, description = "Username")
    ),
    security(
    ()
    )
)]
pub async fn user_id(
    Extension(state): Extension<Arc<State>>,
    username: Path<String>,
) -> SResult<Json<u64>> {
    let user = user_by_username(state, &username.0)
        .await
        .ok()
        .flatten()
        .ok_or_else(|| ServerError::NotFound(Cow::from("user"), Cow::from(username.0)))?;
    Ok(Json(user.id))
}

#[instrument]
#[utoipa::path(
    get,
    path = "/users/profile_picture/{id}",
    responses(
    (status = 200, description = "User Profile Picture URL", body = u64),
    (status = 404, description = "User does not exist/no profile picture"),
    (status = 500, description = "Failed")),
    params(
    ("id" = u64, path, description = "User ID")
    ),
    security(
    ()
    )
)]
pub async fn profile_picture(
    Extension(state): Extension<Arc<State>>,
    id: Path<u64>,
) -> SResult<String> {
    let user = user_by_id(state, id.0).await?;
    user.profile_picture.ok_or_else(|| {
        ServerError::NotFound(Cow::from("profile picture"), Cow::from(id.0.to_string()))
    })
}

#[instrument]
#[utoipa::path(
    get,
    path = "/users/account_creation_date/{id}",
    responses(
    (status = 200, description = "Account Creation UTC Timestamp", body = i64),
    (status = 404, description = "User does not exist."),
    (status = 500, description = "Failed")),
    params(
    ("id" = u64, path, description = "User ID")
    ),
    security(
    ()
    )
)]
pub async fn get_account_creation_date(
    Extension(state): Extension<Arc<State>>,
    id: Path<u64>,
) -> SResult<Json<i64>> {
    let user = user_by_id(state, id.0).await?;
    Ok(Json(user.creation_date.timestamp()))
}

#[instrument]
#[utoipa::path(
    get,
    path = "/users/user_data",
    responses(
    (status = 200, description = "Data for user", body = UserData),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist."),
    (status = 500, description = "Failed")),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn get_user_data(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
) -> SResult<Json<UserData>> {
    let userdata = user_data_by_user_id(state.clone(), user.into()).await?;
    Ok(Json(userdata.into_userdata()))
}

#[instrument]
#[utoipa::path(
    patch,
    path = "/users/set_user_data",
    request_body = UserData,
    responses(
    (status = 200, description = "Sucessfully Updated"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist."),
    (status = 500, description = "Failed")),
    security(
    ()
    )
)]
pub async fn patch_set_user_data(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Json(new_data): Json<UserData>,
) -> SResult<()> {
    update_user_data_by_user_id(state, user.into(), new_data).await?;
    Ok(())
}

route! {
    "/username/:id" => get(username),
    "/user_id/:username" => get(user_id),
    "/profile_picture/:id" => get(profile_picture),
    "/account_creation_date/:id" => get(get_account_creation_date),
    "/user_data/:id" => get(get_user_data),
    "/set_user_data" => patch(patch_set_user_data)
}
