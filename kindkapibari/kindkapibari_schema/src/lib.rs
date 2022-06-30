use crate::error::ServerError;
use axum_core::response::IntoResponse;
pub mod access;
pub mod appdata_traits;
pub mod error;
pub mod redis;
pub mod schema;

/// handler error type
pub type HResult<T>
where
    T: IntoResponse,
= axum_core::response::Result<T, ServerError>;

pub type SResult<T> = axum_core::response::Result<T, ServerError>;

#[macro_export]
macro_rules! opt_to_sr {
    ($opt:expr) => {
        match $opt {
            Option::Some(s) => s,
            Option::None => $crate::ServerError::NotFound("".into(), "".into()),
        }
    };
}
