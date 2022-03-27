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

    #[oai(path = "/pack_by_id", method = "get")]
    async fn pack_by_id(&self, id: Query<Uuid>) -> Json<Option<coconutpak::Model>> {}

    #[oai(path = "/pack_by_name", method = "get")]
    async fn pack_by_name(&self, name: Query<String>) -> Json<Vec<coconutpak::Model>> {}

    #[oai(path = "/pack_by_name", method = "get")]
    async fn 
}
