use poem_openapi::OpenApi;

pub mod coconutpak;
pub mod user;

struct Api;

#[OpenApi]
impl Api {}
