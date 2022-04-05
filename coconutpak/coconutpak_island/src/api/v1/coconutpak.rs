use crate::schema::*;
use poem::Result;
use poem_openapi::payload::Attachment;
use poem_openapi::{auth::ApiKey, param::Query, payload::Json, Multipart, OpenApi};
use uuid::Uuid;

#[derive(Debug, Multipart)]
struct FileUpload {
    name: String,
    data: Vec<u8>,
}

struct CoconutPakApi;

#[OpenApi]
impl CoconutPakApi {
    // query and metadata

    #[oai(path = "/query", method = "get")]
    async fn query(&self, name: Query<String>, sort: Query<i8>) -> Json<Vec<coconutpak::Model>> {
        //
    }

    #[oai(path = "/pakid2name", method = "get")]
    async fn pack_name_from_id(&self, id: Query<u64>) -> Result<Json<String>> {}

    #[oai(path = "/pak/:name/data", method = "get")]
    async fn pack_data_id(&self, name: Path<String>) -> Result<Json<coconutpak::Model>> {}

    #[oai(path = "/pak/:name/versions", method = "get")]
    async fn pack_versions(
        &self,
        name: Path<String>,
    ) -> Result<Json<Vec<coconutpak_history::Model>>> {
    }

    // file/pak related

    #[oai(path = "/upload", method = "post")]
    async fn upload(&self, auth: ApiKey, data: FileUpload) -> Result<Json<u64>> {}

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

    #[oai(path = "/pak/:name/:version/readme", method = "get")]
    async fn readme(
        &self,
        name: Path<String>,
        version: Path<String>,
    ) -> Result<Attachment<Vec<u8>>> {
    }
}
