use crate::login::generate_id;
use crate::permissions::Scopes::Report;
use crate::{
    access,
    eyre::Report,
    schema::{coconutpak, coconutpak_versions, reports},
    AppData, SResult,
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
use tracing::log::{error, log, warn};
#[instrument]
pub async fn get_coconut_pak_id_by_name(state: Arc<AppData>, name: String) -> SResult<Option<u64>> {
    match state
        .redis
        .get::<&str, u64>(concat!("cpk:name2id:", name))
        .await
    {
        Ok(model) => {
            access::refresh_redis_cache(state, concat!("cpk:name2id:", name).to_string(), None);
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

            access::insert_into_cache_with_timeout(
                state,
                concat!("cpk:name2id:", name).to_string(),
                &pak,
                None,
            );
            pak
        }
    }
}

#[instrument]
pub async fn get_coconut_pak(state: Arc<AppData>, id: u64) -> SResult<Option<coconutpak::Model>> {
    match state
        .redis
        .get::<&str, Option<coconutpak::Model>>(concat!("cpk:byid:", id))
        .await
    {
        Ok(model) => {
            access::refresh_redis_cache(state, concat!("cpk:byid:", id).to_string(), None);
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

            access::insert_into_cache_with_timeout(
                state,
                concat!("cpk:bn:", name).to_string(),
                &pak,
                None,
            );
            pak
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
            access::refresh_redis_cache(state, concat!("cpk:vers:", id).to_string(), Some(60));
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
                .ok_or(Report::msg(format!(
                    "CoconutPak with ID {id} does not exist."
                )))?;

            let versions: Vec<coconutpak_versions::Model> = pak
                .find_related(coconutpak_versions::Entity)
                .all(&state.database)
                .await?;
            access::insert_into_cache_with_timeout(
                state,
                concat!("cpk:vers:", id).to_string(),
                &versions,
                Some(60),
            );
            Ok(versions)
        }
    }
}

#[instrument]
pub async fn get_coconut_pak_version(
    state: Arc<AppData>,
    pak_id: u64,
    version: String,
) -> SResult<Option<coconutpak_versions::Model>> {
    log!(
        "get_coconut_pak_version: ",
        version_tag = %version,
        coconutpak = %pak_id,
    );
    if let Err(why) = Version::parse(&version) {
        error!(
            "get_coconut_pak_version: invalid version",
            version_tag = %version,
            coconutpak = %pak_id,
            error = ?why,
        );
        return Err(Report::msg("Invalid Version Tag"));
    }

    let mut pak_versions = get_coconut_pak_versions(state, pak_id).await?;
    pak_versions.retain(|ver| ver.version == version);

    Ok(pak_versions.pop())
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
        if let Err(why) = Version::parse(&version) {
            log!(
                "get_coconut_pak_reports: ",
                version_tag = %version,
                coconutpak = ?pak_id,
                user_id = ?user_id,
            );
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
        None => {
            return Err(Report::msg(format!(
                "CoconutPak with ID {id} and version {version} does not exist."
            )))
        }
    }
}

#[instrument]
pub async fn patch_coconut_pak_yank(
    state: Arc<AppData>,
    pak_id: u64,
    version: String,
    user: u64,
) -> SResult<()> {
    log!(
        "post_coconut_pak_yank: ",
        user = %user,
        pak = %pak_id,
        version = %version,
    );

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
    log!(
        "post_coconut_pak_unyank: ",
        user = %user,
        pak = %pak_id,
        version = %version,
    );

    let mut active_pak_version = get_coconut_pak_version(state.clone(), pak_id, version)
        .await?
        .ok_or(Report::msg(format!(
            "CoconutPak with ID {id} and version {version} does not exist."
        )))?
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
