use crate::{AppData, SResult};
use color_eyre::Report;
use poem::error::NotFound;
use poem_openapi::response::StaticFileResponse::InternalServerError;
use redis::{AsyncCommands, RedisResult, ToRedisArgs};
use sea_orm::{
    query::*, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::sync::Arc;
use tracing::{error, warn};

pub mod api_key;
pub mod bans;
pub mod coconutpak;
pub mod coconutpak_data;
pub mod coconutpak_versions;
pub mod reports;
pub mod session;
pub mod subscribers;
pub mod user;

pub async fn get_coconut_pak_id_by_name(state: Arc<AppData>, name: String) -> SResult<Option<u64>> {
    match state
        .redis
        .get::<&str, u64>(concat!("cpk:name2id:", name))
        .await
    {
        Ok(model) => {
            refresh_redis_cache(state, concat!("cpk:name2id:", name).to_string(), None);
            Ok(Some(model))
        }
        Err(why_redis) => {
            error!(
                "redis cache: get_coconut_pak_id_by_name: ",
                argument = %name,
                error = ?why_redis,
            );

            let pak = coconutpak::Entity::find()
                .filter(coconutpak::Column::Name.eq(&name))
                .one(&state.database)
                .await
                .map_err(|dberr| {
                    error!(
                        "postgres : get_coconut_pak_by_name: ",
                        argument = %name,
                        error = ?dberr,
                    );
                    dberr
                })?
                .map(|pak| pak.id);

            insert_into_cache_with_timeout(
                state,
                concat!("cpk:name2id:", name).to_string(),
                &pak,
                None,
            );
            pak
        }
    }
}

pub async fn get_coconut_pak(state: Arc<AppData>, id: u64) -> SResult<Option<coconutpak::Model>> {
    match state
        .redis
        .get::<&str, Option<coconutpak::Model>>(concat!("cpk:byid:", id))
        .await
    {
        Ok(model) => {
            refresh_redis_cache(state, concat!("cpk:byid:", id).to_string(), None);
            Ok(model)
        }
        Err(why_redis) => {
            warn!(
                "redis cache: get_coconut_pak: ",
                argument = %id,
                error = ?why_redis,
            );

            let pak = coconutpak::Entity::find_by_id(id)
                .one(&state.database)
                .await
                .map_err(|dberr| {
                    error!(
                        "postgres: get_coconut_pak: ",
                        argument = %id,
                        error = ?dberr,
                    );
                    dberr
                })?;

            insert_into_cache_with_timeout(state, concat!("cpk:bn:", name).to_string(), &pak, None);
            pak
        }
    }
}

pub async fn get_coconut_pak_versions(
    state: Arc<AppData>,
    id: u64,
) -> SResult<Vec<coconutpak_versions::Model>> {
    match state
        .redis
        .get::<&str, Vec<coconutpak_versions::Model>>(concat!("cpk:vers:", id))
        .await
    {
        Ok(versions) => {
            refresh_redis_cache(state, concat!("cpk:vers:", id).to_string(), Some(60));
            Ok(versions)
        }
        Err(why) => {
            warn!(
                "redis cache: get_coconut_pak_versions: ",
                argument = %id,
                error = ?why,
            );

            let pak = get_coconut_pak(state.clone(), id)
                .await?
                .ok_or(NotFound(Report::msg(format!(
                    "CoconutPak with ID {id} does not exist."
                ))))?;

            let versions: Vec<coconutpak_versions::Model> = pak
                .find_related(coconutpak_versions::Entity)
                .all(&state.database)
                .await?;
            insert_into_cache_with_timeout(
                state,
                concat!("cpk:vers:", id).to_string(),
                &versions,
                Some(60),
            );
            Ok(versions)
        }
    }
}

pub async fn get_coconut_pak_readme(state: Arc<AppData>, id: u64) -> SResult<Option<String>> {
    todo!()
}

pub async fn get_coconut_pak_reports(
    state: Arc<AppData>,
    pak_id: u64,
    user_id: u64,
    version: String,
) -> SResult<Vec<reports::Model>> {
}
pub async fn post_coconut_pak_report(
    state: Arc<AppData>,
    pak_id: u64,
    user_id: u64,
    version: String,
    reason: String,
) -> SResult<()> {
}

// we just say "fuck it" when handling redis errors in code
// if we get an error we just log it since postgres will pick up the slack
fn insert_into_cache_with_timeout(
    state: Arc<AppData>,
    key: String,
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

fn refresh_redis_cache(state: Arc<AppData>, arg: String, timeout: Option<usize>) {
    tokio::task::spawn(ref_red_cac_raw(state, arg, timeout));
}

async fn ref_red_cac_raw(state: Arc<AppData>, arg: String, timeout: Option<usize>) {
    if let Err(why) = state.redis.expire(&arg, timeout.unwrap_or(360)).await {
        error!(
            format!("redis timeout error: {arg}"),
            argument = %arg,
            error = ?why,
        );
    }
}
