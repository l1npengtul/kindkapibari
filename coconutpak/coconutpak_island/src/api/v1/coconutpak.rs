use crate::schema::*;
use crate::AppData;
use poem::Result;
use poem_openapi::payload::Attachment;
use poem_openapi::{auth::ApiKey, param::Query, payload::Json, Multipart, OpenApi};
use sea_orm::{ColumnTrait, QueryFilter};
use std::sync::Arc;

#[derive(Debug, Multipart)]
struct FileUpload {
    name: String,
    data: Vec<u8>,
}

struct CoconutPakApi {
    data: Arc<AppData>,
}

#[OpenApi(prefix_path = "/v1/coconutpaks", tag = "super::VersionTags::V1")]
impl CoconutPakApi {
    // query and metadata

    #[oai(path = "/query", method = "get")]
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
    #[oai(path = "/pak/:name/data", method = "get")]
    async fn pack_data_name(&self, name: Path<String>) -> Result<Json<coconutpak::Model>> {
        match coconutpak::Entity::find()
            .filter(coconutpak::Column::Name.eq(name))
            .one(&self.data.database)
            .await
            .ok()
            .flatten() {}
    }

    #[oai(path = "/pak/:name/versions", method = "get")]
    async fn pack_versions(
        &self,
        name: Path<String>,
    ) -> Result<Json<Vec<coconutpak_history::Model>>> {
    }

    #[oai(path = "/pak/:name/:version/readme", method = "get")]
    async fn readme(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<Vec<u8>>> {
    }

    // file/pak related

    #[oai(path = "/pak/:name/:version/yank", method = "post")]
    async fn yank(&self, auth: ApiKey, state: Query<bool>) -> Result<Json<bool>> {}

    #[oai(path = "/pak/:name/:version/download", method = "get")]
    async fn download(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<Vec<u8>>> {
    }

    #[oai(path = "/pak/:name/:version/source", method = "get")]
    async fn source(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<Vec<u8>>> {
    }

    #[oai(path = "/upload", method = "post")]
    async fn upload(&self, auth: ApiKey, data: FileUpload) -> Result<Json<u64>> {}
}
