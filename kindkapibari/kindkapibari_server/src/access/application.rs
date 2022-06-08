use crate::applications::Model;
use crate::{schema::applications, AppData, SResult, ServerError};
use redis::{AsyncCommands, ControlFlow, Msg};
use sea_orm::{ActiveValue, EntityTrait};
use std::fmt::Display;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn application_by_id(state: Arc<AppData>, id: u64) -> SResult<applications::Model> {
    let application_query = applications::Entity::find_by_id(id)
        .one(&state.database)
        .await?;
    // commit to cache
    state
        .caches
        .applications
        .insert(id, application_query.clone()); // rip alloc
    Ok(application_query.ok_or(ServerError::NotFound("No application" as dyn Display, "Not Found" as dyn Display))?)
}

pub fn invalidate_application_cache(state: Arc<AppData>, msg: Msg) -> ControlFlow<()> {
    if let Ok(id) = msg.get_pattern::<u64>() {
        state.caches.applications.blocking_invalidate(id);
    }
    ControlFlow::Continue
}

#[instrument]
pub async fn new_application(
    state: Arc<AppData>,
    application: applications::ActiveModel,
) -> SResult<u64> {
    let mut application = application;
    let new_id = state.id_generator.generate_id();
    application.id = ActiveValue::Set(new_id);
    application.insert(&state.database).await?;
    state.redis.publish("APPLICATION_CACHE", new_id);
    Ok(new_id)
}
