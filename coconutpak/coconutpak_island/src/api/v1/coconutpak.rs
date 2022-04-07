use crate::api::v1::CoconutPakUserApiKey;
use crate::report::Report;
use crate::schema::coconutpak::Model;
use crate::schema::*;
use crate::AppData;
use chrono::{DateTime, Utc};
use color_eyre::eyre;
use poem::error::{InternalServerError, NotFound, NotFoundError};
use poem::Result;
use poem_openapi::param::Path;
use poem_openapi::payload::{Attachment, PlainText};
use poem_openapi::{auth::ApiKey, param::Query, payload::Json, Multipart, OpenApi};
use redis::{AsyncCommands, Commands};
use sea_orm::{
    ActiveValue, ColumnTrait, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::future::join;
use std::sync::Arc;

#[derive(Debug, Multipart)]
struct FileUpload {
    name: String,
    data: Vec<u8>,
}

struct CoconutPakApi {
    data: Arc<AppData>,
}

// #[OpenApi(prefix_path = "/v1/coconutpaks", tag = "super::VersionTags::V1")]
impl CoconutPakApi {
    // query and metadata

    // #[oai(path = "/query", method = "get")]
    async fn query(&self, name: Query<String>) -> Json<Vec<coconutpak::SearchableCoconutPak>> {
        let search_result = self
            .data
            .meilisearch
            .index("coconutpaks")
            .search()
            .with_query(&name)
            .execute::<coconutpak::SearchableCoconutPak>()
            .await
            .unwrap()
            .hits
            .into_iter()
            .map(|pak| pak.result)
            .collect::<Vec<coconutpak::SearchableCoconutPak>>();
        Json(search_result)
    }

    // i am a lazy asshole
    // fuck redis, im just using postgres raw :sunglasses:
    // >mfw 3 months from release im desperately adding caching
    // because the response time is 60 seconds because postgres decided
    // to be a piece of shit
    // #[oai(path = "/pak/:name/data", method = "get")]
    async fn pack_data_name(&self, name: Path<String>) -> Result<Json<coconutpak::Model>> {
        let pak = match self.get_pak_from_name(name.0).await? {
            Some(pak) => pak,
            None => return Err(NotFound(eyre::Report::msg("Pak Not Found".to_string()))),
        };

        Ok(Json(pak))
    }

    // #[oai(path = "/pak/:name/versions", method = "get")]
    async fn pack_versions(
        &self,
        name: Path<String>,
    ) -> Result<Json<Vec<coconutpak_history::Model>>> {
        let pak = self
            .get_pak_from_name(name.0)
            .await?
            .ok_or(NotFound(eyre::Report::msg("Pak Not Found".to_string())))?;

        let pak_versions = coconutpak_history::Entity::find()
            .filter(coconutpak_history::Column::Coconutpak.eq(pak.id))
            .join(
                JoinType::RightJoin,
                coconutpak_history::Relation::CoconutPak.def(),
            )
            .all(&self.data.database)
            .await
            .unwrap_or_default();
        return Ok(Json(pak_versions));
    }

    // #[oai(path = "/pak/:name/:version/data", method = "get")]
    async fn pack_version_data(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Json<coconutpak_history::Model>> {
    }

    // #[oai(path = "/pak/:name/:version/readme", method = "get")]
    async fn readme(&self, name: Path<String>, version: Path<String>) -> Result<PlainText<String>> {
        return Ok(PlainText("".to_string()));
    }

    // #[oai(path = "/pak/:name/:version/report", method = "post")]
    async fn report(
        &self,
        name: Path<String>,
        version: Path<String>,
        report: PlainText<String>,
        api_key: CoconutPakUserApiKey,
    ) -> Result<()> {
        let pak = self
            .get_pak_from_name(name.0)
            .await?
            .ok_or(NotFound(eyre::Report::msg("Pak Not Found".to_string())))?;

        let report_active = reports::ActiveModel {
            reporter: ActiveValue::Set(api_key.0.uuid),
            target_pak: ActiveValue::Set(pak.id),
            date: ActiveValue::Set(Utc::now()),
            reason: ActiveValue::Set(report.0),
            version: ActiveValue::Set(version.0),
        };

        return match report_active.insert(&self.data.database).await {
            Ok(_) => Ok(()),
            Err(why) => Err(InternalServerError(why.into())),
        };
    }

    // file/pak related

    // #[oai(path = "/pak/:name/:version/yank", method = "post")]
    async fn yank(
        &self,
        name: Path<String>,
        version: Path<String>,
        auth: CoconutPakUserApiKey,
        state: Query<bool>,
    ) -> Result<()> {
        let pak = self
            .get_pak_from_name(name.0)
            .await?
            .ok_or(NotFound(eyre::Report::msg("Pak Not Found".to_string())))?;
    }

    // #[oai(path = "/pak/:name/:version/download", method = "get")]
    async fn download(
        &self,
        name: Path<String>,
        version: Path<String>,
        api_key: CoconutPakUserApiKey,
    ) -> Result<Attachment<Vec<u8>>> {
    }

    // #[oai(path = "/pak/:name/:version/downloadnoverify", method = "get")]
    async fn download_no_verify(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<Vec<u8>>> {
    }

    // #[oai(path = "/pak/:name/:version/source", method = "get")]
    async fn source(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<Vec<u8>>> {
    }

    // #[oai(path = "/upload", method = "post")]
    async fn upload(&self, auth: CoconutPakUserApiKey, data: FileUpload) -> Result<Json<u64>> {}
}

impl CoconutPakApi {
    async fn get_pak_from_name(&self, name: String) -> Result<Option<coconutpak::Model>> {
        // redis check
        if let Ok(cache_result) = self
            .data
            .redis
            .get::<&'_ str, Option<coconutpak::Model>>(concat!("coconutpak:paks:", name))
            .await
        {
            return Ok(cache_result);
        }

        return match coconutpak::Entity::find()
            .filter(coconutpak::Column::Name.eq(name))
            .one(&self.data.database)
            .await
        {
            Ok(pak) => {
                let _redisresult = self
                    .data
                    .redis
                    .set(concat!("coconutpak:paks:", name), &pak)
                    .await;
                let _redisresult = self
                    .data
                    .redis
                    .expire(concat!("coconutpak:paks:", name), 3600)
                    .await;
                Ok(pak)
            }
            Err(why) => Err(InternalServerError(why)),
        };
    }
}
