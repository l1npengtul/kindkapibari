use std::sync::Arc;
use redis::{AsyncCommands, RedisResult};
use tracing::error;
use crate::{AppData, SResult};

pub mod api_key;
pub mod bans;
pub mod coconutpak;
pub mod coconutpak_data;
pub mod coconutpak_history;
pub mod reports;
pub mod session;
pub mod subscribers;
pub mod user;

pub async fn get_coconut_pak_by_name(state: Arc<AppData>, name: String) -> SResult<Option<coconutpak::Model>> {
    match state.redis.get::<&str, Option<coconutpak::Model>>(concat!("cpk:bn:", name)).await {
        Ok(model) => Ok(model),
        Err(why_redis) => {
            error!(
                "get_coconut_pak_by_name",
                argument = %"name",
                error = ?why_redis,
            );
            Ok(None)
        }
    }

}

pub async fn get_coconut_pak_history(state: Arc<AppData>, id: u64)
