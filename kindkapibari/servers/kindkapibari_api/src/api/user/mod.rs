// use kindkapibari_core::route;

pub mod oauth;
pub mod onetime;
pub mod recurring;
pub mod sober;
pub mod users;

// route! {
//     onetime,
//     recurring,
//     sober,
//     users
// }

#[must_use]
pub fn routes() -> axum::Router {
    axum::Router::new()
        .merge(onetime::routes())
        .merge(recurring::routes())
        .merge(sober::routes())
        .merge(users::routes())
}
