use crate::{
    access::{insert_into_cache_with_timeout, login::generate_id, refresh_redis_cache},
    eyre::Report,
    schema::{coconutpak, coconutpak_versions, reports},
    AppData, SResult, ServerError,
};
use chrono::Utc;
use redis::AsyncCommands;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};
use semver::Version;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
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
        Err(_) => {
            let pak = coconutpak::Entity::find()
                .filter(coconutpak::Column::Name.eq(&name))
                .one(&state.database)
                .await?
                .map(|pak| pak.id);

            insert_into_cache_with_timeout(state, concat!("cpk:name2id:", name), &pak, None);
            pak
        }
    }
}

#[instrument]
pub async fn get_coconut_pak(state: Arc<AppData>, id: u64) -> SResult<coconutpak::Model> {
    match state
        .redis
        .get::<&str, Option<coconutpak::Model>>(concat!("cpk:byid:", id))
        .await
    {
        Ok(model) => {
            refresh_redis_cache(state, concat!("cpk:byid:", id), None);
            Ok(model.ok_or(ServerError::NotFound(format!("{id}"), "None"))?)
        }
        Err(_) => {
            let pak = coconutpak::Entity::find_by_id(id)
                .one(&state.database)
                .await?;
            insert_into_cache_with_timeout(state, concat!("cpk:bn:", name), &pak, None);
            Ok(pak.ok_or(ServerError::NotFound(format!("{id}"), "None"))?)
        }
    }
}

#[instrument]
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
            refresh_redis_cache(state, concat!("cpk:vers:", id), Some(60));
            Ok(versions)
        }
        Err(_) => {
            let pak = get_coconut_pak(state.clone(), id)
                .await?
                .ok_or(Report::msg(format!(
                    "CoconutPak with ID {id} does not exist."
                )))?;

            let versions: Vec<coconutpak_versions::Model> = pak
                .find_related(coconutpak_versions::Entity)
                .all(&state.database)
                .await?;
            insert_into_cache_with_timeout(state, concat!("cpk:vers:", id), &versions, Some(60));
            Ok(versions)
        }
    }
}

#[instrument]
pub async fn get_coconut_pak_version(
    state: Arc<AppData>,
    pak_id: u64,
    version: String,
) -> SResult<coconutpak_versions::Model> {
    if let Err(why) = Version::parse(&version) {
        return Err(ServerError::BadArgumentError("version", why));
    }

    let mut pak_versions = get_coconut_pak_versions(state, pak_id).await?;
    pak_versions.retain(|ver| ver.version == version);

    Ok(pak_versions
        .pop()
        .ok_or(ServerError::NotFound(format!("{pak_id}@{version}"), "None"))?)
}

#[instrument]
pub async fn get_coconut_pak_readme(state: Arc<AppData>, id: u64) -> SResult<Option<String>> {
    todo!()
}

#[instrument]
pub async fn get_coconut_pak_reports(
    state: Arc<AppData>,
    pak_id: u64,
    user_id: Option<u64>,
    version: Option<String>,
) -> SResult<Vec<reports::Model>> {
    let mut report_query = coconutpak::Entity::find_by_id(pak_id)
        .join(JoinType::RightJoin, reports::Relation::CoconutPak.def());

    if let Some(user_id) = user_id {
        report_query = report_query.filter(reports::Column::Reporter.eq(user_id));
    }

    if let Some(version) = version {
        if let Err(_) = Version::parse(&version) {
            return Err(ServerError::BadArgumentError("version", why));
        }

        report_query = report_query.filter(reports::Column::Version.eq(version));
    }

    let reports = report_query
        .into_model::<reports::Model>()
        .all(&state.database)
        .await?;

    Ok(reports)
}

#[instrument]
pub async fn post_coconut_pak_report(
    state: Arc<AppData>,
    pak_id: u64,
    user_id: u64,
    version: String,
    reason: String,
) -> SResult<reports::Model> {
    match get_coconut_pak_version(state.clone(), pak_id, version.clone()).await? {
        Some(ver) => {
            let report_active = reports::ActiveModel {
                report_id: ActiveValue::NotSet,
                reporter: ActiveValue::Set(user_id),
                target_pak: ActiveValue::Set(ver.coconutpak),
                date: ActiveValue::Set(Utc::now()),
                reason: ActiveValue::Set(reason),
                version: ActiveValue::Set(version),
            };

            let report = report_active.insert(&state.database).await?;
            Ok(report)
        }
        None => return Err(ServerError::NotFound("coconutpak", pak_id)),
    }
}

#[instrument]
pub async fn patch_coconut_pak_yank(
    state: Arc<AppData>,
    pak_id: u64,
    version: String,
    user: u64,
) -> SResult<()> {
    let mut active_pak_version = get_coconut_pak_version(state.clone(), pak_id, version)
        .await?
        .ok_or(Report::msg(format!(
            "CoconutPak with ID {id} and version {version} does not exist."
        )))?
        .into_active_model();

    // TODO: Log!
    active_pak_version.yanked = ActiveValue::Set(true);
    coconutpak_versions::Entity::insert(active_pak_version)
        .exec(&state.database)
        .await?;
    Ok(())
}

#[instrument]
pub async fn patch_coconut_pak_unyank(
    state: Arc<AppData>,
    pak_id: u64,
    version: String,
    user: u64,
) -> SResult<()> {
    let mut active_pak_version = get_coconut_pak_version(state.clone(), pak_id, version)
        .await?
        .into_active_model();

    active_pak_version.yanked = ActiveValue::Set(false);
    coconutpak_versions::Entity::update(active_pak_version)
        .exec(&state.database)
        .await?;
    Ok(())
}

#[instrument]
pub async fn get_coconut_pak_files(
    state: Arc<AppData>,
    pak_id: u64,
    version: u64,
) -> SResult<Vec<u8>> {
    todo!()
}

#[instrument]
pub async fn post_coconut_pak_publish(
    state: Arc<AppData>,
    user_id: u64,
    data: Vec<u8>,
) -> SResult<u64> {
    let new_pak_id = generate_id(state.clone())
        .await
        .ok_or(Report::msg("Failed to generate ID"))?;

    // TODO: submit API
    todo!()
}
