use crate::State;
use chrono::Utc;
use kindkapibari_core::reminder::{OneTimeReminder, OneTimeReminders};
use kindkapibari_schema::{
    error::ServerError,
    schema::users::{onetime_reminders, user},
    SResult,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
pub async fn get_onetime_reminders(
    state: Arc<State>,
    user: user::Model,
) -> SResult<OneTimeReminders> {
    let onetimes: Vec<onetime_reminders::Model> = user
        .find_related(onetime_reminders::Entity)
        .all(&state.database)
        .await?;

    let now = Utc::now();
    let mut one_time = Vec::with_capacity(onetimes.len());
    for reminder in onetimes {
        if reminder.expire < now {
            let st_cl = state.clone();
            tokio::task::spawn(async move {
                delete_onetime_reminder(st_cl, user.id, reminder.id).await
            });
        } else {
            one_time.push(OneTimeReminder {
                id: reminder.id,
                name: reminder.name,
                set: reminder.set,
                expire: reminder.expire,
            });
        }
    }

    one_time.shrink_to_fit();

    Ok(OneTimeReminders { one_time })
}

#[instrument]
pub async fn get_onetime_reminder(
    state: Arc<State>,
    user: u64,
    one_time: u64,
) -> SResult<onetime_reminders::Model> {
    let one_time = onetime_reminders::Entity::find_by_id(one_time)
        .one(&state.database)
        .await?
        .ok_or_else(|| {
            ServerError::NotFound(
                Cow::from("one time reminder"),
                Cow::from(format!("{one_time:?}")),
            )
        })?;

    if one_time.owner != user {
        return Err(ServerError::NotFound(
            Cow::from("one time reminder"),
            Cow::from(format!("{one_time:?}")),
        ));
    }

    if one_time.expire > Utc::now() {
        let stcl = state.clone();
        tokio::task::spawn(async move { delete_onetime_reminder(stcl, user, one_time.id).await });
        return Err(ServerError::NotFound(
            Cow::from("one time reminder"),
            Cow::from(format!("{one_time:?}")),
        ));
    }

    Ok(one_time)
}

#[instrument]
async fn get_onetime_raw_nochk(
    state: Arc<State>,
    user: u64,
    one_time: u64,
) -> SResult<onetime_reminders::Model> {
    let one_time = onetime_reminders::Entity::find_by_id(one_time)
        .one(&state.database)
        .await?
        .ok_or_else(|| {
            ServerError::NotFound(
                Cow::from("one time reminder"),
                Cow::from(format!("{one_time:?}")),
            )
        })?;

    if one_time.owner != user {
        return Err(ServerError::NotFound(
            Cow::from("one time reminder"),
            Cow::from(format!("{one_time:?}")),
        ));
    }

    Ok(one_time)
}

#[instrument]
async fn get_onetime_reminder_raw(
    state: Arc<State>,
    user: u64,
    one_time: u64,
) -> SResult<onetime_reminders::Model> {
    let one_time = onetime_reminders::Entity::find_by_id(one_time)
        .one(&state.database)
        .await?
        .ok_or_else(|| {
            ServerError::NotFound(
                Cow::from("one time reminder"),
                Cow::from(format!("{one_time:?}")),
            )
        })?;

    if one_time.owner != user {
        return Err(ServerError::NotFound(
            Cow::from("one time reminder"),
            Cow::from(format!("{one_time:?}")),
        ));
    }

    Ok(one_time)
}

#[instrument]
async fn check_if_onetime_already_exists(
    state: Arc<State>,
    user: user::Model,
    new_name: &String,
) -> SResult<bool> {
    if get_onetime_reminders(state.clone(), user)
        .await?
        .one_time
        .into_iter()
        .map(|one_time| one_time.name)
        .any(|x| x == *new_name)
    {
        return Ok(true);
    }
    Ok(false)
}

#[instrument]
pub async fn update_onetime_reminder(
    state: Arc<State>,
    user: user::Model,
    reminder: OneTimeReminder,
) -> SResult<()> {
    if reminder.name.len() > 160 || reminder.expire > Utc::now() || reminder.expire <= reminder.set
    {
        return Err(ServerError::BadRequest(Cow::from("invalid reminder")));
    }

    let current_reminder = get_onetime_reminder(state.clone(), user.id, reminder.id).await?;
    if current_reminder.name == reminder.name && current_reminder.expire == reminder.expire {
        return Ok(());
    }

    if check_if_onetime_already_exists(state.clone(), user, &reminder.name).await? {
        return Err(ServerError::BadRequest(Cow::from("already exist!")));
    }

    let mut onetime_active_mdl = current_reminder.into_active_model();
    onetime_active_mdl.name = ActiveValue::Set(reminder.name);
    onetime_active_mdl.expire = ActiveValue::Set(reminder.expire);
    onetime_active_mdl.update(&state.database).await?;

    Ok(())
}

#[instrument]
pub async fn add_onetime_reminder(
    state: Arc<State>,
    user: user::Model,
    reminder: OneTimeReminder,
) -> SResult<u64> {
    if reminder.name.len() > 160 || reminder.expire > Utc::now() || reminder.expire <= reminder.set
    {
        return Err(ServerError::BadRequest(Cow::from("invalid reminder")));
    }

    let id = user.id;

    if check_if_onetime_already_exists(state.clone(), user, &reminder.name).await? {
        return Err(ServerError::BadRequest(Cow::from("already exist!")));
    }

    let reminder_id = state.id_generator.onetime_reminder_ids.generate_id();

    let reminder_active = onetime_reminders::ActiveModel {
        id: ActiveValue::Set(reminder_id),
        owner: ActiveValue::Set(id),
        name: ActiveValue::Set(reminder.name),
        set: ActiveValue::Set(reminder.set),
        expire: ActiveValue::Set(reminder.expire),
    };

    reminder_active.insert(&state.database).await?;
    Ok(reminder_id)
}

#[instrument]
pub async fn delete_onetime_reminder(state: Arc<State>, user: u64, reminder: u64) -> SResult<()> {
    let onetime = get_onetime_raw_nochk(state.clone(), user, reminder).await?;

    onetime.delete(&state.database).await?;

    Ok(())
}
