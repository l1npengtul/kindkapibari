use crate::State;
use chrono::{Duration, Utc};
use kindkapibari_core::{
    reminder::{
        u8_bitflag_to_days, OneTimeReminder, OneTimeReminders, RecurringReminder,
        RecurringReminders,
    },
    sober::{Sober, Sobers},
    user_data::UserData,
};
use kindkapibari_schema::{
    error::ServerError,
    schema::users::{onetime_reminders, recurring_reminders, sobers, user, userdata},
    SResult,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, ModelTrait,
    QueryFilter, QuerySelect, RelationTrait,
};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
pub async fn user_by_id(state: Arc<State>, id: u64) -> SResult<user::Model> {
    // check our local cache
    if let Some(possible_user) = state.caches.users_cache.get(&id) {
        return Ok(possible_user);
    }

    match user::Entity::find_by_id(id).one(&state.database).await? {
        Some(u) => {
            state.cache().insert(id, u.clone()).await;
            Ok(u)
        }
        None => Err(ServerError::NotFound(Cow::from("user"), Cow::from("id"))),
    }
}

#[instrument]
pub async fn user_by_username(state: Arc<State>, name: &str) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(name))
        .one(state.database())
        .await?;
    Ok(user)
}

#[instrument]
pub async fn user_by_email(state: Arc<State>, email: &str) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(state.database())
        .await?;
    Ok(user)
}

#[instrument]
pub async fn user_data_by_user_id(state: Arc<State>, user: u64) -> SResult<userdata::Model> {
    let userdata = user::Entity::find_by_id(user)
        .join(JoinType::LeftJoin, userdata::Relation::User.def())
        .into_model::<userdata::Model>()
        .one(state.database())
        .await?
        .ok_or(ServerError::NotFound(
            Cow::from("userdata"),
            Cow::from(user.to_string()),
        ))?;
    Ok(userdata)
}

#[instrument]
pub async fn update_user_data_by_user_id(
    state: Arc<State>,
    user: u64,
    userdata: UserData,
) -> SResult<()> {
    let user = user_data_by_user_id(state.clone(), user).await?;
    let mut user_data_active: userdata::ActiveModel = user.into();
    user_data_active.locale = ActiveValue::Set(userdata.locale);
    user_data_active.birthday = ActiveValue::Set(userdata.birthday);
    user_data_active.gender = ActiveValue::Set(userdata.gender);
    user_data_active.pronouns = ActiveValue::Set(userdata.pronouns);
    user_data_active.update(state.database()).await?;
    Ok(())
}

#[instrument]
pub async fn get_sobers_by_user_id(state: Arc<State>, user: u64) -> SResult<Sobers> {
    let user = user_by_id(state.clone(), user).await?;
    let sobers: Vec<sobers::Model> = user
        .find_related(sobers::Entity)
        .all(state.database())
        .await?;
    let sobers = sobers
        .into_iter()
        .map(|sober_mdl| Sober {
            id: sober_mdl.id,
            name: sober_mdl.name,
            start_time: sober_mdl.time_since_reset,
        })
        .collect::<Vec<Sober>>();
    Ok(Sobers { sobers })
}
#[instrument]
pub async fn get_sober(state: Arc<State>, user: u64, sober: u64) -> SResult<sobers::Model> {
    let sobers = sobers::Entity::find_by_id(sober)
        .one(&state.database)
        .await?
        .ok_or(ServerError::NotFound(
            Cow::from("sober"),
            Cow::from(format!("{sober}")),
        ))?;

    if sobers.owner != user {
        return Err(ServerError::NotFound(
            Cow::from("sober"),
            Cow::from(format!("{sober}")),
        ));
    }

    Ok(sobers)
}

#[instrument]
async fn check_if_sober_already_exists(
    state: Arc<State>,
    sober_name: &String,
    user: u64,
) -> SResult<bool> {
    let sobers = get_sobers_by_user_id(state.clone(), user)
        .await?
        .sobers
        .into_iter()
        .map(|sober| sober.name)
        .collect::<Vec<String>>();
    if sobers.contains(sober_name) {
        return Ok(true);
    }
    return Ok(false);
}

#[instrument]
pub async fn reset_sober_by_name_and_user_id(
    state: Arc<State>,
    sober: u64,
    user: u64,
) -> SResult<i64> {
    let sobers = get_sober(state.clone(), user, sober).await?;

    let timestamp = match sobers {
        Some(sober) => {
            let mut sober_active_mdl = sober.into_active_model();
            let new_time = Utc::now();
            sober_active_mdl.time_since_reset = ActiveValue::Set(new_time.clone());
            sober_active_mdl.update(&state.database).await?;
            new_time.timestamp_millis()
        }
        None => {
            return Err(ServerError::NotFound(
                Cow::from("sober"),
                Cow::from(format!("{sober}")),
            ))
        }
    };

    Ok(timestamp)
}

#[instrument]
pub async fn update_sober_name_by_user_id(
    state: Arc<State>,
    sober_id: u64,
    new_name: String,
    user_id: u64,
) -> SResult<()> {
    let current_sober = get_sober(state.clone(), user_id, sober_id).await?;

    if !check_if_sober_already_exists(state.clone(), &new_name, user_id).await? {
        return Err(ServerError::BadRequest(Cow::from("already exists!")));
    }

    if current_sober.name == new_name {
        return Ok(());
    }

    let mut sober_active_mdl = current_sober.into_active_model();
    sober_active_mdl.name = ActiveValue::Set(new_name);
    sober_active_mdl.update(&state.database).await?;

    Ok(())
}

#[instrument]
pub async fn add_sober_by_user(state: Arc<State>, user: u64, new_sober: Sober) -> SResult<()> {
    // start time is stamped on the user's device
    // use 5 seconds to compensate for ping, etc...
    if now - new_sober.start_time > Duration::seconds(5) || new_sober.start_time > Utc::now() {
        return Err(ServerError::BadRequest(Cow::from("bad time")));
    }

    if new_sober.name.len().count() > 160 {
        return Err(ServerError::BadRequest(Cow::from("too long!")));
    }

    if !check_if_sober_already_exists(state.clone(), &new_sober.name, user).await? {
        return Err(ServerError::BadRequest(Cow::from("already exists!")));
    }

    let _ = user_by_id(state.clone(), user).await?;

    let sober_id = state.id_generator.sober_ids.generate_id();

    let sober_active = sobers::ActiveModel {
        id: ActiveValue::Set(sober_id),
        owner: ActiveValue::Set(user),
        name: ActiveValue::Set(new_sober.name),
        time_since_reset: ActiveValue::Set(new_sober.start_time),
    };

    sober_active.insert(state.database()).await?;
    Ok(())
}

#[instrument]
pub async fn delete_sober_by_user_id(state: Arc<State>, user: u64, sober: u64) -> SResult<()> {
    let sober = get_sober(state.clone(), user, sober).await?;

    sober.delete(&state.database).await?;

    Ok(())
}

#[instrument]
pub async fn get_onetime_reminders_by_user_id(
    state: Arc<State>,
    user: u64,
) -> SResult<OneTimeReminders> {
    let user = user_by_id(state.clone(), user).await?;
    let onetimes: Vec<onetime_reminders::Model> = user
        .find_related(onetime_reminders::Entity)
        .all(&state.database)
        .await?;

    let now = Utc::now();
    let mut one_time = Vec::with_capacity(onetimes.len());
    for reminder in onetimes {
        if reminder.expire < now {
            tokio::task::spawn(async {
                delete_onetime_reminder_by_user_id(state.clone(), user.id, reminder.id).await
            })
        } else {
            one_time.push(OneTimeReminder {
                id: reminder.id,
                name: reminder.name,
                set: reminder.set,
                expire: reminder.expire,
            })
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
        .ok_or(ServerError::NotFound(
            Cow::from("one time reminder"),
            Cow::from(format!("{one_time}")),
        ))?;

    if one_time.owner != user {
        return Err(ServerError::NotFound(
            Cow::from("one time reminder"),
            Cow::from(format!("{one_time}")),
        ));
    }

    if one_time.expire > Utc::now() {
        tokio::task::spawn(async {
            delete_onetime_reminder_by_user_id(state, user, one_time.id).await
        });
        return Err(ServerError::NotFound(
            Cow::from("one time reminder"),
            Cow::from(format!("{one_time}")),
        ));
    }

    Ok(one_time)
}

#[instrument]
async fn check_if_onetime_already_exists(
    state: Arc<State>,
    user: u64,
    new_name: &String,
) -> SResult<bool> {
    let one_time = get_onetime_reminders_by_user_id(state.clone(), user)
        .await?
        .one_time
        .into_iter()
        .map(|one_time| one_time.name)
        .collect::<Vec<String>>();
    if one_time.contains(new_name) {
        return Ok(true);
    }
    return Ok(false);
}

#[instrument]
pub async fn update_onetime_reminder_by_user_id(
    state: Arc<State>,
    user: u64,
    reminder: OneTimeReminder,
) -> SResult<()> {
    if reminder.name.len() > 160 || reminder.expire > Utc::now() || reminder.expire <= reminder.set
    {
        return Err(ServerError::BadRequest(Cow::from("invalid reminder")));
    }

    let current_reminder = get_onetime_reminder(state.clone(), user, reminnder.id).await?;
    if current_reminder == reminder {
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
pub async fn add_onetime_reminder_by_user_id(
    state: Arc<State>,
    user: u64,
    reminder: OneTimeReminder,
) -> SResult<()> {
    if reminder.name.len() > 160 || reminder.expire > Utc::now() || reminder.expire <= reminder.set
    {
        return Err(ServerError::BadRequest(Cow::from("invalid reminder")));
    }

    if check_if_onetime_already_exists(state.clone(), user, &reminder.name).await? {
        return Err(ServerError::BadRequest(Cow::from("already exist!")));
    }

    let _ = user_by_id(state.clone(), user).await?;

    let reminder_id = state.id_generator.onetime_reminder_ids.generate_id();

    let reminder_active = onetime_reminders::ActiveModel {
        id: ActiveValue::Set(reminder_id),
        owner: ActiveValue::Set(user),
        name: ActiveValue::Set(reminder.name),
        set: ActiveValue::Set(reminder.set),
        expire: ActiveValue::Set(reminder.expire),
    };

    reminder_active.insert(&state.database).await?;
    Ok(())
}

#[instrument]
pub async fn delete_onetime_reminder_by_user_id(
    state: Arc<State>,
    user: u64,
    reminder: u64,
) -> SResult<()> {
    let onetime = get_onetime_reminder(state.clone(), user, reminder).await?;

    onetime.delete(&state.database).await?;

    Ok(())
}

#[instrument]
pub async fn get_recurring_reminders(state: Arc<State>, user: u64) -> SResult<RecurringReminders> {
    let user = user_by_id(state.clone(), user).await?;
    let mut recurring: Vec<recurring_reminders::Model> = user
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
        .ok_or(ServerError::NotFound(
            Cow::from("recurring reminder"),
            Cow::from(format!("{recurring_reminder}")),
        ))?;

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
    user: u64,
    new_name: &String,
) -> SResult<bool> {
    let recurring = get_recurring_reminders(state.clone(), user)
        .await?
        .recurring
        .into_iter()
        .map(|recur| recur.name)
        .collect::<Vec<String>>();
    if recurring.contains(new_name) {
        return Ok(true);
    }
    return Ok(false);
}
