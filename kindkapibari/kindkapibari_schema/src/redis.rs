use crate::appdata_traits::AppDataRedis;
use crate::SResult;
use redis::{FromRedisValue, ToRedisArgs};
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn insert_into_cache(
    state: Arc<impl AppDataRedis>,
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
    state: Arc<impl AppDataRedis>,
    arg: impl ToRedisArgs,
    timeout: Option<usize>,
) -> SResult<()> {
    Ok(state.redis.expire(&arg, timeout.unwrap_or(360)).await?)
}

#[instrument]
pub async fn delet_dis<F: FromRedisValue>(
    state: Arc<impl AppDataRedis>,
    arg: impl ToRedisArgs,
) -> SResult<F> {
    Ok(state.redis.del(arg)?)
}

#[instrument]
pub async fn check_if_exists_cache(state: Arc<impl AppDataRedis>, data: impl ToRedisArgs) -> bool {
    state.redis.get(data).await.is_ok()
}

#[instrument]
pub async fn read_from_cache<T>(state: Arc<impl AppDataRedis>, key: impl ToRedisArgs) -> SResult<T>
where
    T: FromRedisArgs,
{
    state.redis.get(key)?
}
