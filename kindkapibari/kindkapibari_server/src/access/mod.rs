use crate::AppData;
use redis::{AsyncCommands, RedisError, ToRedisArgs};
use std::sync::Arc;
use tokio::time::timeout;
use tracing::{error, instrument};

pub mod login;
pub mod oauth;
pub mod oauth_thirdparty;

pub const TOKEN_SEPERATOR: &'static str = "-";

// we just say "fuck it" when handling redis errors in code
// if we get an error we just log it since postgres will pick up the slack

#[instrument]
pub async fn insert_into_cache(
    state: Arc<AppData>,
    key: impl ToRedisArgs,
    value: impl ToRedisArgs,
    timeout: Option<usize>,
) {
    tokio::task::spawn(async move {
        if let Err(why) = state.redis.set(&key, value).await {
            error!(
                format!("redis timeout error: {key}"),
                argument = %key,
                error = ?why,
            );
        }
        ref_red_cac_raw(state, key, timeout).await;
    });
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
pub async fn delet_dis(state: Arc<AppData>, arg: impl ToRedisArgs) {
    state.redis.del(arg)?
}
