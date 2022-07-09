use crate::{
    access::{
        recurring::{
            add_recurring_reminder, delete_recurring_reminder, get_recurring_reminders,
            update_recurring_reminder,
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
    reminder::{RecurringReminder, RecurringReminders},
    roles::Role,
    route,
};
use kindkapibari_schema::{error::ServerError, SResult};
use std::sync::Arc;
use tracing::instrument;

#[instrument]
#[utoipa::path(
    get,
    path = "/users/recurring_reminders/{user_id}",
    responses(
    (status = 200, description = "Sucessfully Updated Recurring Reminder"),
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
pub async fn get_user_recurring_reminders(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Path(user_id): Path<u64>,
) -> SResult<Json<RecurringReminders>> {
    let mut user = user;
    if user.roles >= Role::Server || user.id != user_id {
        return Err(ServerError::Unauthorized);
    }

    user = user_by_id(state.clone(), user_id).await?.into();
    let recurring = get_recurring_reminders(state, user.into()).await?;
    Ok(Json(recurring))
}

#[instrument]
#[utoipa::path(
    patch,
    path = "/users/update_recurring_reminder",
    request_body = RecurringReminder,
    responses(
    (status = 200, description = "Sucessfully Updated Recurring Reminder"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Reminder does not exist"),
    (status = 500, description = "Failed")),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn patch_update_recurring_reminders(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Json(recurring): Json<RecurringReminder>,
) -> SResult<()> {
    update_recurring_reminder(state, user.into(), recurring).await?;
    Ok(())
}

#[instrument]
#[utoipa::path(
    post,
    path = "/users/add_recurring_reminders",
    request_body = RecurringReminder,
    responses(
    (status = 200, description = "Sucessfully Updated Recurring Reminder, new reminder ID", body = u64),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Reminder does not exist"),
    (status = 500, description = "Failed")),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn post_add_recurring_reminder(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    Json(recurring): Json<RecurringReminder>,
) -> SResult<Json<u64>> {
    let id = add_recurring_reminder(state, user.into(), recurring).await?;
    Ok(Json(id))
}

#[instrument]
#[utoipa::path(
    delete,
    path = "/users/delete_recurring_reminders/{recurring_reminder_id}",
    request_body = RecurringReminder,
    responses(
    (status = 200, description = "Sucessfully Deleted Recurring Reminder"),
    (status = 401, description = "Bad Token"),
    (status = 403, description = "Bad Token"),
    (status = 404, description = "User does not exist/Reminder does not exist"),
    (status = 500, description = "Failed")),
    params(
    ("recurring_reminder_id" = u64, path, description = "Recurring Reminder ID")
    ),
    security(
    ("api_jwt_token" = []),
    )
)]
pub async fn delete_user_recurring_reminder(
    Extension(state): Extension<Arc<State>>,
    Authentication(user): Authentication<UserAuthMdl>,
    recurring_reminder_id: Path<u64>,
) -> SResult<()> {
    delete_recurring_reminder(state, user.id, recurring_reminder_id.0).await?;
    Ok(())
}

route! {
    "/recurring_reminders/:user_id" => get(get_user_recurring_reminders),
    "/update_recurring_reminder" => patch(patch_update_recurring_reminders),
    "/add_recurring_reminders" => post(post_add_recurring_reminder),
    "/delete_recurring_reminders/:recurring_reminder_id" => delete(delete_user_recurring_reminder)
}
