use axum::routing::{delete, get, post, put};
use axum::Router;

pub async fn get_user_settings() {}
pub async fn post_user_settings() {}
pub async fn put_user_settings() {}

pub async fn get_user_by_uuid() {}

pub async fn get_user_connections() {}

pub fn users(router: Router) -> Router {
    router
        .route(
            "/api/users/settings",
            get(get_user_settings)
                .post(post_user_settings)
                .put(put_user_settings),
        )
        .route("/api/users/uuid", get(get_user_by_uuid))
}
