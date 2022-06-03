use std::sync::Arc;
use sea_orm::{ActiveValue, EntityTrait};
use tracing::instrument;
use crate::{AppData, schema::applications, ServerError, SResult};

#[instrument]
pub async fn application_by_id(state: Arc<AppData>, id: u64) -> SResult<applications::Model> {
    if let Ok(application) = state.caches.applications.get(&id) {
        return Ok(application);
    }

    let application_query = applications::Entity::find_by_id(id)
        .one(&state.database)
        .await?;
    // commit to cache
    state
        .caches
        .applications
        .insert(id, application_query.clone()); // rip alloc
    Ok(application_query.ok_or(ServerError::NotFound("No application", "Not Found"))?)
}

#[instrument]
pub async fn invalidate_cache(state: Arc<AppData>) {
    state.redis.
}

#[instrument]
pub async fn new_application(state: Arc<AppData>, application: applications::ActiveModel) -> SResult<u64> {
    let mut application = application;
    let new_id = state.id_generator.generate_id();
    application.id = ActiveValue::Set(new_id);

}