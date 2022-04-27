use crate::{
    access::{insert_into_cache_with_timeout, refresh_redis_cache},
    schema::{coconutpak, subscribers, user},
    AppData, SResult,
};
use redis::{AsyncCommands, RedisResult};
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::sync::Arc;
use tracing::instrument;
use tracing::log::{error, log, warn};

#[instrument]
pub async fn get_user_subscribes(
    state: Arc<AppData>,
    user_id: u64,
) -> SResult<Vec<coconutpak::Model>> {
    match state.redis.get::<&str, Option<Vec<>>>()

    let subscribed_paks = user::Entity::find_by_id(user_id)
        .join(JoinType::RightJoin, subscribers::Relation::User.def())
        .join(JoinType::RightJoin, subscribers::Relation::CoconutPak.def())
        .into_model::<coconutpak::Model>()
        .all(&state.database)
        .await?;

    Ok(subscribed_paks)
}

#[instrument]
pub async fn post_user_subscribes(state: Arc<AppData>, user_id: u64, pak_id: u64) -> SResult<()> {
    let active_subscribe = subscribers::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        pak_id: ActiveValue::Set(pak_id),
    };

    active_subscribe.insert(&state.database).await?;
    Ok(())
}

#[instrument]
pub async fn get_user_published_coconut_paks(
    state: Arc<AppData>,
    user_id: u64,
) -> SResult<Vec<coconutpak::Model>> {
    match state
        .redis
        .get::<&str, Option<Vec<coconutpak::Model>>>(concat!("cpk:uspks:", user_id))
        .await
    {
        Ok(user_paks) => {
            refresh_redis_cache(state, concat!("cpk:uspks:", user_id).to_string(), Some(20));
            Ok(user_paks.unwrap_or_default())
        }
        Err(why) => {
            warn!(
                "redis cache: get_coconut_pak_versions: ",
                argument = %id,
                error = ?why,
            );

            let user_paks = coconutpak::Entity::find()
                .filter(coconutpak::Column::Owner.eq(user_id))
                .all(&state.database)
                .await?;

            insert_into_cache_with_timeout(
                state,
                concat!("cpk:uspks:", user_id).to_string(),
                &user_paks,
                Some(20),
            );

            Ok(user_paks)
        }
    }
}
