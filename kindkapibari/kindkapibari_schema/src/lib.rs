#![deny(clippy::pedantic)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]

use crate::error::ServerError;

pub mod error;
pub mod redis;
pub mod schema;

/// handler error type
pub type HResult<T> = axum_core::response::Result<T, ServerError>;
pub type SResult<T> = axum_core::response::Result<T, ServerError>;

#[macro_export]
macro_rules! opt_to_sr {
    ($opt:expr) => {
        match $opt {
            Option::Some(s) => s,
            Option::None => return Err(ServerError::NotFound("".into(), "".into())),
        }
    };
    ($opt:expr, $r1:expr, $r2:expr) => {
        match $opt {
            Option::Some(s) => s,
            Option::None => return Err(ServerError::NotFound($r1.into(), $r2.into())),
        }
    };
}
