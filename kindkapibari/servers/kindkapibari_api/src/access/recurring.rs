use crate::State;
use kindkapibari_core::reminder::{
    days_to_u8, u8_bitflag_to_days, RecurringReminder, RecurringReminders,
};
use kindkapibari_schema::{
    error::ServerError,
    schema::users::{recurring_reminders, user},
    SResult,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
pub async fn get_recurring_reminders(
    state: Arc<State>,
    user: user::Model,
) -> SResult<RecurringReminders> {
    let recurring: Vec<recurring_reminders::Model> = user
        .find_related(recurring_reminders::Entity)
        .all(&state.database)
        .await?;

    let recurring = recurring
        .into_iter()
        .map(|reminder_mdl| RecurringReminder {
            id: reminder_mdl.id,
            name: reminder_mdl.name,
            time: reminder_mdl.time,
            days: u8_bitflag_to_days(reminder_mdl.days),
        })
        .collect::<Vec<RecurringReminder>>();

    Ok(RecurringReminders { recurring })
}

#[instrument]
pub async fn get_recurring_reminder(
    state: Arc<State>,
    user: u64,
    recurring_reminder: u64,
) -> SResult<recurring_reminders::Model> {
    let reminder = recurring_reminders::Entity::find_by_id(recurring_reminder)
        .one(&state.database)
        .await?
        .ok_or_else(|| {
            ServerError::NotFound(
                Cow::from("recurring reminder"),
                Cow::from(format!("{recurring_reminder}")),
            )
        })?;

    if reminder.owner != user {
        return Err(ServerError::NotFound(
            Cow::from("recurring reminder"),
            Cow::from(format!("{recurring_reminder}")),
        ));
    }

    Ok(reminder)
}

#[instrument]
async fn check_if_reminder_already_exists(
    state: Arc<State>,
    user: user::Model,
    new_name: &String,
) -> SResult<bool> {
    if get_recurring_reminders(state.clone(), user)
        .await?
        .recurring
        .into_iter()
        .map(|recur| recur.name)
        .any(|x| x == *new_name)
    {
        return Ok(true);
    }
    Ok(false)
}

#[instrument]
pub async fn update_recurring_reminder(
    state: Arc<State>,
    user: user::Model,
    updated_reminder: RecurringReminder,
) -> SResult<()> {
    if updated_reminder.name.len() > 160 {
        return Err(ServerError::BadRequest(Cow::from(
            "invalid recurring reminder",
        )));
    }

    let updated_date_u8 = days_to_u8(updated_reminder.days);

    let current_reminder =
        get_recurring_reminder(state.clone(), user.id, updated_reminder.id).await?;
    if current_reminder.name == updated_reminder.name
        && current_reminder.days == updated_date_u8
        && current_reminder.time == updated_reminder.time
    {
        return Ok(());
    }

    if check_if_reminder_already_exists(state.clone(), user, &updated_reminder.name).await? {
        return Err(ServerError::BadRequest(Cow::from("already exist!")));
    }

    let mut recurring_active_mdl = current_reminder.into_active_model();
    recurring_active_mdl.name = ActiveValue::Set(updated_reminder.name);
    recurring_active_mdl.days = ActiveValue::Set(updated_date_u8);
    recurring_active_mdl.time = ActiveValue::Set(updated_reminder.time);
    recurring_active_mdl.update(&state.database).await?;

    Ok(())
}

#[instrument]
pub async fn add_recurring_reminder(
    state: Arc<State>,
    user: user::Model,
    new_reminder: RecurringReminder,
) -> SResult<u64> {
    if new_reminder.name.len() > 160 {
        return Err(ServerError::BadRequest(Cow::from(
            "invalid recurring reminder",
        )));
    }

    let uid = user.id;

    if check_if_reminder_already_exists(state.clone(), user, &new_reminder.name).await? {
        return Err(ServerError::BadRequest(Cow::from("already exist!")));
    }

    let new_id = state.id_generator.recurring_reminder_ids.generate_id();

    let recurring_active = recurring_reminders::ActiveModel {
        id: ActiveValue::Set(new_id),
        owner: ActiveValue::Set(uid),
        name: ActiveValue::Set(new_reminder.name),
        days: ActiveValue::Set(days_to_u8(new_reminder.days)),
        time: ActiveValue::Set(new_reminder.time),
    };

    recurring_active.insert(&state.database).await?;
    Ok(new_id)
}

#[instrument]
pub async fn delete_recurring_reminder(state: Arc<State>, user: u64, reminder: u64) -> SResult<()> {
    let recurring = get_recurring_reminder(state.clone(), user, reminder).await?;

    recurring.delete(&state.database).await?;

    Ok(())
}
