use crate::{AppData, SResult};
use redis::{AsyncCommands, RedisError, ToRedisArgs};
use std::sync::Arc;
use tracing::instrument;
use tracing::log::{error, log};

pub mod coconutpak;
pub mod login;
pub mod user;

// we just say "fuck it" when handling redis errors in code
// if we get an error we just log it since postgres will pick up the slack

#[instrument]
pub async fn insert_without_timeout(
    state: Arc<AppData>,
    key: impl ToRedisArgs,
    value: impl ToRedisArgs,
) -> Result<(), RedisError> {
    if let Err(why) = state.redis.set(&key, value).await {
        error!(
            format!("redis timeout error: {key}"),
            argument = %key,
            error = ?why,
        );
        return Err(why);
    }
    Ok(())
}

#[instrument]
pub fn insert_into_cache_with_timeout(
    state: Arc<AppData>,
    key: impl ToRedisArgs,
    value: impl ToRedisArgs,
    timeout: Option<usize>,
) {
    tokio::task::spawn(async move || {
        if let Err(why) = state.redis.set(&key, value).await {
            error!(
                format!("redis timeout error: {key}"),
                argument = %key,
                error = ?why,
            );
            return;
        }
        ref_red_cac_raw(state, key, timeout).await;
    });
}

#[instrument]
pub fn refresh_redis_cache(state: Arc<AppData>, arg: impl ToRedisArgs, timeout: Option<usize>) {
    tokio::task::spawn(ref_red_cac_raw(state, arg, timeout));
}

#[instrument]
pub async fn ref_red_cac_raw(state: Arc<AppData>, arg: impl ToRedisArgs, timeout: Option<usize>) {
    if let Err(why) = state.redis.expire(&arg, timeout.unwrap_or(360)).await {
        error!(
            format!("redis timeout error: {arg}"),
            argument = %arg,
            error = ?why,
        );
    }
}

#[instrument]
pub fn invalidate_cache(state: Arc<AppData>, arg: impl ToRedisArgs) {
    tokio::task::spawn(delet_dis(state, arg));
}

#[instrument]
pub async fn delet_dis(state: Arc<AppData>, arg: impl ToRedisArgs) {
    state.redis.del(arg)?
}
