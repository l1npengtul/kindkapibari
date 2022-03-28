use poem_openapi::{
    auth::ApiKey,
    param::Query,
    payload::Json,
    Multipart,
    OpenApi
};
use poem::Result;
use crate::schema::*;
use uuid::Uuid;

#[derive(Debug, Multipart)]
struct FileUpload {
    name: String,
    data: Vec<u8>
}

struct CoconutPakApi;

#[OpenApi]
impl CoconutPakApi {
    #[oai(path = "/query", method = "get")]
    async fn query(&self, name: Query<String>, sort: Query<i8>) -> Json<Vec<coconutpak::Model>> {
        //
    }

    #[oai(path = "/pack/data_name/:name", method = "get")]
    async fn pack_data_name(&self, name: Path<String>) -> Result<Json<coconutpak::Model>> {}

    #[oai(path = "/pack/data/:id", method = "get")]
    async fn pack_data_id(&self, id: Path<Uuid>) -> Result<Json<coconutpak::Model>> {}

    async fn pack_versions(&self, id: Path<Uuid>) -> Result<Json<Vec<coconutpak_history::Model>>> {}

    #[oai(path = "/upload", method = "post")]
    async fn upload(&self, auth: ApiKey, data: FileUpload) -> Result<>
}
