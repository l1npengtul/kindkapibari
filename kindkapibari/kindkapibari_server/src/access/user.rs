use std::sync::Arc;
use sea_orm::EntityTrait;
use tracing::instrument;
use crate::{AppData, ServerError, SResult, user};
use crate::user::Model;

#[instrument]
pub async fn get_user_by_id(state: Arc<AppData>, id: u64) -> SResult<user::Model> {
    // check our local cache
    if let Some(possible_user) = state.caches.users.get(&id) {
        return possible_user.ok_or(ServerError::NotFound("user", id));
    }

    let user = user::Entity::find_by_id(id).one(&state.database).await?;
    state.caches.users.insert(id, user.clone()).await;
    return user.ok_or(ServerError::NotFound("user", id));
}

pub struct UserCreationRequest {
    pub display: Option<String>,
    pub handle: String,
    pub
}

pub fn create_user