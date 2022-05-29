use crate::{AppData, SResult};
use redis::{AsyncCommands, FromRedisValue, ToRedisArgs};
use std::sync::Arc;
use tracing::{error, instrument};

pub mod auth;
pub mod user;

pub const TOKEN_SEPERATOR: &'static str = "-";

// we just say "fuck it" when handling redis errors in code
// if we get an error we just log it since postgres will pick up the slack

#[instrument]
pub async fn insert_into_cache(
    state: Arc<AppData>,
    key: impl ToRedisArgs,
    value: impl ToRedisArgs,
    timeout: Option<usize>,
) -> SResult<()> {
    state.redis.set(&key, value).await?;
    if !timeout.is_none() {
        ref_red_cac_raw(state, key, timeout).await?;
    }
    Ok(())
}

#[instrument]
pub async fn ref_red_cac_raw(
    state: Arc<AppData>,
    arg: impl ToRedisArgs,
    timeout: Option<usize>,
) -> SResult<()> {
    Ok(state.redis.expire(&arg, timeout.unwrap_or(360)).await?)
}

#[instrument]
pub async fn delet_dis<F: FromRedisValue>(
    state: Arc<AppData>,
    arg: impl ToRedisArgs,
) -> SResult<F> {
    Ok(state.redis.del(arg)?)
}
