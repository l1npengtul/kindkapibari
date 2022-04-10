use crate::api::v1::CoconutPakUserAuthentication;
use crate::report::Report;
use crate::schema::coconutpak::Model;
use crate::schema::*;
use crate::AppData;
use chrono::{DateTime, Utc};
use color_eyre::eyre;
use kindkapibari_core::throttle::ThrottledBytes;
use kindkapibari_core::version::Version;
use poem::error::{Forbidden, InternalServerError, NotFound, NotFoundError, NotImplemented};
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
use tokio::io::AsyncReadExt;

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

    // #[oai(path = "/search", method = "get")]
    async fn search(&self, name: Query<String>) -> Json<Vec<coconutpak::SearchableCoconutPak>> {
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
        api_key: CoconutPakUserAuthentication,
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
        auth: CoconutPakUserAuthentication,
    ) -> Result<()> {
        let pak = self
            .get_pak_from_name(name.0)
            .await?
            .ok_or(NotFound(eyre::Report::msg("Pak Not Found".to_string())))?;
        if pak.owner != auth.0.uuid {
            return Err(Forbidden(eyre::Report::msg(
                "You do not own this CoconutPak.",
            )));
        }
        let parsed_version = Version::parse(&version)?;
        let mut pak_versions = coconutpak_history::Entity::find()
            .filter(coconutpak_history::Column::Coconutpak.eq(pak.id))
            .join(
                JoinType::RightJoin,
                coconutpak_history::Relation::CoconutPak.def(),
            )
            .all(&self.data.database)
            .await
            .unwrap_or_default();
        pak_versions.retain(|x| x.version == parsed_version);
        return if let Some(paks) = pak_versions.get(0) {
            if pak_versions.len() == 1 {
                let mut active_pak: coconutpak_history::ActiveModel = paks.into();
                active_pak.yanked = ActiveValue::Set(true);
                if let Err(why) = active_pak.update(&self.data.database).await {
                    return Err(InternalServerError(why.into()));
                }
                Ok(())
            } else {
                Err(InternalServerError(eyre::Report::msg(
                    "Too many CoconutPaks!",
                )))
            }
        } else {
            Err(NotFound(eyre::Report::msg("CoconutPak Not Found.")))
        };
    }

    // #[oai(path = "/pak/:name/:version/download", method = "get")]
    async fn download(
        &self,
        name: Path<String>,
        version: Path<String>,
        _api_key: CoconutPakUserAuthentication,
    ) -> Result<Attachment<ThrottledBytes>> {
        let pak = self
            .get_pak_from_name(name.0)
            .await?
            .ok_or(NotFound(eyre::Report::msg("Pak Not Found".to_string())))?;
        let mut pak_versions = coconutpak_history::Entity::find()
            .filter(coconutpak_history::Column::Coconutpak.eq(pak.id))
            .join(
                JoinType::RightJoin,
                coconutpak_history::Relation::CoconutPak.def(),
            )
            .all(&self.data.database)
            .await
            .unwrap_or_default();
        pak_versions.retain(|x| x.version == parsed_version);
        if let Some(paks) = pak_versions.get(0) {
            if !paks.yanked {
                let path = &self
                    .data
                    .config
                    .read()
                    .await
                    .compiler
                    .compile_target_directory
                    .to_owned();
                let compiled =
                    match tokio::fs::File::open(path + format!("{name}/{version}/{name}.cpak"))
                        .await
                    {
                        Ok(mut f) => {
                            let mut data = Vec::new();
                            f.read(&mut data).await.map_err(|_| {
                                InternalServerError(eyre::Report::msg("Failed to open compiled."))
                            })?;
                            data
                        }
                        Err(_) => {
                            return Err(NotFound(eyre::Report::msg(
                                "Not yet compiled or does not exist.",
                            )))
                        }
                    };
                return Ok(Attachment::new(ThrottledBytes::new(compiled, 0)));
            }
        }
        return Err(NotFound(eyre::Report::msg("Pak Not Found.")));
    }

    // #[oai(path = "/pak/:name/:version/downloadnoverify", method = "get")]
    async fn download_no_verify(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<ThrottledBytes>> {
        Err(Forbidden(eyre::Report::msg(
            "You need to be logged in to do that.",
        )))
    }

    // #[oai(path = "/pak/:name/:version/source", method = "get")]
    async fn source(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<Vec<u8>>> {
        Err(NotImplemented(eyre::Report::msg("Sorry!")))
    }

    // #[oai(path = "/pak/:name/:version/sourcenoverify", method = "get")]
    async fn source_no_verify(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<ThrottledBytes>> {
        Err(Forbidden(eyre::Report::msg(
            "You need to be logged in to do that.",
        )))
    }

    // #[oai(path = "/pak/:name/upload", method = "post")]
    async fn upload(
        &self,
        auth: CoconutPakUserAuthentication,
        name: Path<String>,
        version: Query<String>,
        data: FileUpload,
    ) -> Result<Json<u64>> {
        let pak = self
            .get_pak_from_name(name.0)
            .await?
            .ok_or(NotFound(eyre::Report::msg("Pak Not Found".to_string())))?;
        let mut pak_versions = coconutpak_history::Entity::find()
            .filter(coconutpak_history::Column::Coconutpak.eq(pak.id))
            .join(
                JoinType::RightJoin,
                coconutpak_history::Relation::CoconutPak.def(),
            )
            .all(&self.data.database)
            .await
            .unwrap_or_default();
    }
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
                    .expire(concat!("coconutpak:paks:", name), 3600)
                    .await;
                Ok(pak)
            }
            Err(why) => Err(InternalServerError(why)),
        };
    }
}
