use crate::access::invalidate_cache;
use crate::access::login::{new_apikey, CoconutPakApiKey};
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
use tracing::log::{error, info, warn};

#[instrument]
pub async fn get_user_subscribes(
    state: Arc<AppData>,
    user_id: u64,
) -> SResult<Vec<coconutpak::Model>> {
    match state
        .redis
        .get::<&str, Vec<coconutpak::Model>>(concat!("cpk:uspk", user_id))
        .await
    {
        Ok(user_paks) => {
            refresh_redis_cache(state, concat!("cpk:uspk:", user_id).to_string(), Some(10));
            Ok(user_paks)
        }
        Err(why) => {
            info!(
                "redis cache: get_user_subscribes: ",
                argument = %id,
                error = ?why,
            );

            let subscribed_paks = user::Entity::find_by_id(user_id)
                .join(JoinType::RightJoin, subscribers::Relation::User.def())
                .join(JoinType::RightJoin, subscribers::Relation::CoconutPak.def())
                .into_model::<coconutpak::Model>()
                .all(&state.database)
                .await?;

            insert_into_cache_with_timeout(
                state,
                concat!("cpk:uspk:", user_id),
                &subscribed_paks,
                Some(20),
            );

            Ok(subscribed_paks)
        }
    }
}

#[instrument]
pub async fn post_user_subscribes(state: Arc<AppData>, user_id: u64, pak_id: u64) -> SResult<()> {
    let active_subscribe = subscribers::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        pak_id: ActiveValue::Set(pak_id),
    };

    active_subscribe.insert(&state.database).await?;
    invalidate_cache(state, concat!("cpk:uspk:", user_id));
    Ok(())
}

#[instrument]
pub async fn get_user_published_coconut_paks(
    state: Arc<AppData>,
    user_id: u64,
) -> SResult<Vec<coconutpak::Model>> {
    match state
        .redis
        .get::<&str, Vec<coconutpak::Model>>(concat!("cpk:uppk:", user_id))
        .await
    {
        Ok(user_paks) => {
            refresh_redis_cache(state, concat!("cpk:uppk:", user_id), Some(20));
            Ok(user_paks)
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
                concat!("cpk:uppk:", user_id),
                &user_paks,
                Some(20),
            );

            Ok(user_paks)
        }
    }
}

#[instrument]
pub async fn get_new_user_api_key(
    state: Arc<AppData>,
    user_id: u64,
    name: String,
) -> SResult<CoconutPakApiKey> {
    return new_apikey(state, user_id, name).await;
}
