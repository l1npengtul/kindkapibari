use crate::State;
use kindkapibari_core::user_data::UserData;
use kindkapibari_schema::{
    error::ServerError,
    schema::users::{user, userdata},
    SResult,
};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
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
            state.caches.users_cache.insert(id, u.clone()).await;
            Ok(u)
        }
        None => Err(ServerError::NotFound(Cow::from("user"), Cow::from("id"))),
    }
}

#[instrument]
pub async fn user_by_username(state: Arc<State>, name: &str) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(name))
        .one(&state.database)
        .await?;
    Ok(user)
}

#[instrument]
pub async fn user_by_email(state: Arc<State>, email: &str) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(&state.database)
        .await?;
    Ok(user)
}

#[instrument]
pub async fn user_data_by_user_id(
    state: Arc<State>,
    user: user::Model,
) -> SResult<userdata::Model> {
    let userdata: userdata::Model = user
        .find_related(userdata::Entity)
        .one(&state.database)
        .await?
        .ok_or_else(|| {
            ServerError::NotFound(Cow::from("userdata"), Cow::from(format!("{}", user.id)))
        })?;
    Ok(userdata)
}

#[instrument]
pub async fn update_user_data_by_user_id(
    state: Arc<State>,
    user: user::Model,
    userdata: UserData,
) -> SResult<()> {
    let user = user_data_by_user_id(state.clone(), user).await?;
    let mut user_data_active: userdata::ActiveModel = user.into();
    user_data_active.locale = ActiveValue::Set(userdata.locale);
    user_data_active.birthday = ActiveValue::Set(userdata.birthday);
    user_data_active.gender = ActiveValue::Set(userdata.gender);
    user_data_active.pronouns = ActiveValue::Set(userdata.pronouns);
    user_data_active.update(&state.database).await?;
    Ok(())
}
