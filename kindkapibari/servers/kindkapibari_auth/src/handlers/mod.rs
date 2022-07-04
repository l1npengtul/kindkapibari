use axum::Router;

pub mod login;
pub mod signup;

#[must_use]
pub fn routes() -> Router {
    Router::new()
        .nest("/login", login::routes())
        .nest("/signup", signup::routes())
}
