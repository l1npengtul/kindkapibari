use poem_openapi::auth::ApiKey;
use crate::schema::*;
use poem_openapi::param::Query;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;
use uuid::Uuid;

struct CoconutPakApi;

#[OpenApi]
impl CoconutPakApi {
    #[oai(path = "/query", method = "get")]
    async fn query(&self, name: Query<String>, sort: Query<i8>) -> Json<Vec<coconutpak::Model>> {
        //
    }

    #[oai(path = "/pack/data_name/:name", method = "get")]
    async fn pack_data_name(&self, name: Path<String>) -> Json<Option<coconutpak::Model>> {}

    #[oai(path = "/pack/data/:id", method = "get")]
    async fn pack_data_id(&self, id: Path<Uuid>) -> Json<Option<coconutpak::Model>> {}

    async fn pack_versions(&self, id: Path<Uuid>) -> Json<Vec<coconutpak_history::Model>> {}

    #[oai(path = "/upload", method = "post")]
    async fn upload(&self, auth: ApiKey, data: )
}
