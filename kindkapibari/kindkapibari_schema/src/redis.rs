use crate::SResult;
use redis::{aio::ConnectionManager, AsyncCommands, FromRedisValue, ToRedisArgs};
use std::{fmt::Debug, sync::Arc};
use tracing::instrument;

pub trait RedisState: Debug + Sized + Send + Sync {
    fn redis(&self) -> &ConnectionManager;
    fn redis_owned(&self) -> ConnectionManager {
        self.redis().clone()
    }
}

#[instrument]
pub async fn insert_into_cache(
    state: Arc<impl RedisState>,
    key: impl ToRedisArgs + Debug + Send + Sync,
    value: impl ToRedisArgs + Debug + Send + Sync,
    timeout: Option<usize>,
) -> SResult<()> {
    state.redis_owned().set(&key, value).await?;
    if timeout.is_some() {
        ref_red_cac_raw(state, key, timeout).await?;
    }
    Ok(())
}

#[instrument]
pub async fn ref_red_cac_raw(
    state: Arc<impl RedisState>,
    arg: impl ToRedisArgs + Debug + Send + Sync,
    timeout: Option<usize>,
) -> SResult<()> {
    Ok(state
        .redis_owned()
        .expire(&arg, timeout.unwrap_or(360))
        .await?)
}

#[instrument]
pub async fn delet_dis<F: FromRedisValue + Debug + Send + Sync>(
    state: Arc<impl RedisState>,
    arg: impl ToRedisArgs + Debug + Send + Sync,
) -> SResult<F> {
    Ok(state.redis_owned().del(arg).await?)
}

#[instrument]
pub async fn check_if_exists_cache<
    K: ToRedisArgs + Debug + Send + Sync,
    V: FromRedisValue + Debug + Send + Sync,
>(
    state: Arc<impl RedisState>,
    key: K,
) -> bool {
    state.redis_owned().exists::<K, V>(key).await.is_ok()
}

#[instrument]
pub async fn read_from_cache<T>(
    state: Arc<impl RedisState>,
    key: impl ToRedisArgs + Debug + Send + Sync,
) -> SResult<T>
where
    T: FromRedisValue + Debug + Send + Sync,
{
    let dbresult: T = state.redis_owned().get(key).await?;
    Ok(dbresult)
}
