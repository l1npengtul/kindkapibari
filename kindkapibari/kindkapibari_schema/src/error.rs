use axum_core::{
    body::boxed,
    response::{IntoResponse, Response},
};
use http::StatusCode;
use redis::{ErrorKind, RedisError};
use sea_orm::error::DbErr;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Could not find {0} with {1}")]
    NotFound(Cow<'static, str>, Cow<'static, str>),
    #[error("Internal Server Database Error.")]
    RedisError(#[from] RedisError),
    #[error("Internal Server Database Error.")]
    DatabaseError(#[from] DbErr),
    #[error("fucky wuckyy uwu :c")]
    InternalServer(Box<dyn std::error::Error + Send + Sync>),
    #[error("Internal Error: {0}")]
    ISErr(Cow<'static, str>),
    #[error("Bad Argument {0}: {1}")]
    BadArgumentError(Cow<'static, str>, Box<dyn std::error::Error + Send + Sync>),
    #[error("Bad Request: {0}")]
    BadRequest(Cow<'static, str>),
    #[error("Unauthorized.")]
    Unauthorized,
    #[error("Forbidden.")]
    Forbidden,
    #[error("Bad Media Type: {0}")]
    BadType(Cow<'static, str>),
    #[error("Too Large!")]
    TooLarge,
    #[error("Stop trying to DDoS me you little shit!")]
    RateLimited,
    #[error("Unavailable for legal reasons. See more here: {0}")]
    LegalReasons(Cow<'static, str>),
    #[error("peng pls fix :ccc (tell her to stop being a lazy shite >:ccc)")]
    NotImplemented,
    #[error("AHHHHHHHHHHHHHHHHHHHHH")]
    Unavailable,
}

// impl From<ServerError> for Error {
//     fn from(why: ServerError) -> Self {
//         match &why {
//             ServerError::NotFound(_, _) => NotFound(why),
//             ServerError::RedisError(_)
//             | ServerError::DatabaseError(_)
//             | ServerError::InternalServer(_) => InternalServerError(why),
//             ServerError::BadArgumentError(_, _) | ServerError::BadRequest(_) => BadRequest(why),
//             ServerError::Unauthorized => Unauthorized(why),
//             ServerError::Forbidden => Forbidden(why),
//             ServerError::BadType(_) => UnsupportedMediaType(why),
//             ServerError::TooLarge => PayloadTooLarge(why),
//             ServerError::RateLimited => TooManyRequests(why),
//             ServerError::LegalReasons(_) => UnavailableForLegalReasons(why),
//             ServerError::NotImplemented => NotImplemented(why),
//             ServerError::Unavailable => ServiceUnavailable(why),
//         }
//     }
// }

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ServerError::NotFound(_, _) => StatusCode::NOT_FOUND,
            ServerError::RedisError(why) => match why.kind() {
                ErrorKind::AuthenticationFailed
                | ErrorKind::BusyLoadingError
                | ErrorKind::InvalidClientConfig => StatusCode::SERVICE_UNAVAILABLE,
                ErrorKind::TypeError => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ServerError::DatabaseError(why) => match why {
                DbErr::RecordNotFound(_) => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            ServerError::InternalServer(_) | ServerError::ISErr(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ServerError::BadArgumentError(_, _) | ServerError::BadRequest(_) => {
                StatusCode::BAD_REQUEST
            }
            ServerError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServerError::Forbidden => StatusCode::FORBIDDEN,
            ServerError::BadType(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ServerError::TooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            ServerError::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            ServerError::LegalReasons(_) => StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
            ServerError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ServerError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        };

        Response::builder()
            .status(status_code)
            .body(boxed(format!("{self}")))
            .unwrap()
    }
}
