use crate::{
    access::{
        onetime::{
            add_onetime_reminder, delete_onetime_reminder, get_onetime_reminders,
            update_onetime_reminder,
        },
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
    reminder::{OneTimeReminder, OneTimeReminders},
    roles::Role,
    route,
};
use kindkapibari_schema::{error::ServerError, SResult};
use std::sync::Arc;
use tracing::instrument;

#[instrument]
#[utoipa::path(
    delete,
    path = "/users/onetime_reminders/{user_id}",
    responses(
    (status = 200, description = "Sucessfully Got Onetime Reminders", body = OneTimeReminders),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Reminder does not exist"),
    (status = 500, description = "Failed")),
    params(
    ("user_id" = u64, path, description = "User ID")
    ),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn get_user_onetime_reminders(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Path(user_id): Path<u64>,
) -> SResult<Json<OneTimeReminders>> {
    let mut user = user;
    if user.roles >= Role::Server || user.id != user_id {
        return Err(ServerError::Unauthorized);
    }
    user = user_by_id(state.clone(), user_id).await?.into();
    let recurring = get_onetime_reminders(state, user.into()).await?;
    Ok(Json(recurring))
}

#[instrument]
#[utoipa::path(
    patch,
    path = "/users/update_onetime_reminders",
    request_body = OneTimeReminder,
    responses(
    (status = 200, description = "Sucessfully Updated Recurring Reminder"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Reminder does not exist"),
    (status = 500, description = "Failed")
    ),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn patch_update_onetime_reminders(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Json(onetime): Json<OneTimeReminder>,
) -> SResult<()> {
    update_onetime_reminder(state, user.into(), onetime).await?;
    Ok(())
}

#[instrument]
#[utoipa::path(
    post,
    path = "/users/add_onetime_reminders",
    request_body = OneTimeReminder,
    responses(
    (status = 200, description = "Sucessfully Added Recurring Reminder, Reminder ID", body = u64),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Reminder does not exist"),
    (status = 500, description = "Failed")),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn post_add_onetime_reminder(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Json(onetime): Json<OneTimeReminder>,
) -> SResult<Json<u64>> {
    let id = add_onetime_reminder(state, user.into(), onetime).await?;
    Ok(Json(id))
}

#[instrument]
#[utoipa::path(
    delete,
    path = "/users/delete_onetime_reminders/{onetime_id}",
    request_body = OneTimeReminder,
    responses(
    (status = 200, description = "Sucessfully Deleted Recurring Reminder"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Reminder does not exist"),
    (status = 500, description = "Failed")),
    params(
    ("onetime_id" = u64, path, description = "Onetime Reminder ID")
    ),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn delete_user_onetime_reminder(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    id: Path<u64>,
) -> SResult<()> {
    delete_onetime_reminder(state, user.id, id.0).await?;
    Ok(())
}

route! {
    "/onetime_reminders/:user_id" => get(get_user_onetime_reminders),
    "/update_onetime_reminders" => patch(patch_update_onetime_reminders),
    "/add_onetime_reminders" => post(post_add_onetime_reminder),
    "/delete_onetime_reminders/:onetime_id" => delete(delete_user_onetime_reminder)
}
