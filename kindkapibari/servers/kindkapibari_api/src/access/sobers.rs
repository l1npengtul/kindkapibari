use crate::State;
use chrono::{Duration, Utc};
use kindkapibari_core::sober::{Sober, Sobers};
use kindkapibari_schema::{
    error::ServerError,
    schema::users::{sobers, user},
    SResult,
};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, ModelTrait};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
pub async fn get_sobers(state: Arc<State>, user: user::Model) -> SResult<Sobers> {
    let sobers: Vec<sobers::Model> = user
        .find_related(sobers::Entity)
        .all(&state.database)
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
        .ok_or_else(|| ServerError::NotFound(Cow::from("sober"), Cow::from(format!("{sober}"))))?;

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
    user: user::Model,
) -> SResult<bool> {
    if get_sobers(state.clone(), user)
        .await?
        .sobers
        .into_iter()
        .map(|sober| sober.name)
        .any(|x| x == *sober_name)
    {
        return Ok(true);
    }
    Ok(false)
}

#[instrument]
pub async fn reset_sober(state: Arc<State>, sober: u64, user: u64) -> SResult<i64> {
    let sobers = get_sober(state.clone(), user, sober).await?;
    let mut sober_active_mdl = sobers.into_active_model();
    let new_time = Utc::now();
    sober_active_mdl.time_since_reset = ActiveValue::Set(new_time);
    sober_active_mdl.update(&state.database).await?;
    Ok(new_time.timestamp_millis())
}

#[instrument]
pub async fn update_sober(
    state: Arc<State>,
    sober_id: u64,
    new_name: String,
    user: user::Model,
) -> SResult<()> {
    let current_sober = get_sober(state.clone(), user.id, sober_id).await?;

    if !check_if_sober_already_exists(state.clone(), &new_name, user).await? {
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
pub async fn add_sober(state: Arc<State>, user: user::Model, new_sober: Sober) -> SResult<u64> {
    // start time is stamped on the user's device
    // use 5 seconds to compensate for ping, etc...
    let now = Utc::now();

    if now - new_sober.start_time > Duration::seconds(5) || new_sober.start_time > now {
        return Err(ServerError::BadRequest(Cow::from("bad time")));
    }

    if new_sober.name.len() > 160 {
        return Err(ServerError::BadRequest(Cow::from("too long!")));
    }

    let uid = user.id;

    if !check_if_sober_already_exists(state.clone(), &new_sober.name, user).await? {
        return Err(ServerError::BadRequest(Cow::from("already exists!")));
    }

    let sober_id = state.id_generator.sober_ids.generate_id();

    let sober_active = sobers::ActiveModel {
        id: ActiveValue::Set(sober_id),
        owner: ActiveValue::Set(uid),
        name: ActiveValue::Set(new_sober.name),
        time_since_reset: ActiveValue::Set(new_sober.start_time),
    };

    sober_active.insert(&state.database).await?;
    Ok(sober_id)
}

#[instrument]
pub async fn delete_sober(state: Arc<State>, user: u64, sober: u64) -> SResult<()> {
    let sober = get_sober(state.clone(), user, sober).await?;

    sober.delete(&state.database).await?;

    Ok(())
}
