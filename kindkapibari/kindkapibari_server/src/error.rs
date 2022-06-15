use crate::SResult;
use poem::error::{
    BadRequest, Forbidden, InternalServerError, NotFound, NotImplemented, PayloadTooLarge,
    ServiceUnavailable, TooManyRequests, Unauthorized, UnavailableForLegalReasons,
    UnsupportedMediaType,
};
use poem::{Error, IntoResponse, Response};
use redis::RedisError;
use sea_orm::error::DbErr;
use std::borrow::Cow;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Could not find {0} with {1}")]
    NotFound(Cow<'static, str>, Cow<'static, str>),
    #[error(transparent)]
    RedisError(#[from] RedisError),
    #[error(transparent)]
    DatabaseError(#[from] DbErr),
    #[error(transparent)]
    InternalServer(#[from] dyn std::error::Error),
    #[error("Bad Argument {0}: {1}")]
    BadArgumentError(Cow<'static, str>, #[from] dyn std::error::Error),
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

impl From<ServerError> for Error {
    fn from(why: ServerError) -> Self {
        match &why {
            ServerError::NotFound(_, _) => NotFound(why),
            ServerError::RedisError(_)
            | ServerError::DatabaseError(_)
            | ServerError::InternalServer(_) => InternalServerError(why),
            ServerError::BadArgumentError(_, _) | ServerError::BadRequest(_) => BadRequest(why),
            ServerError::Unauthorized => Unauthorized(why),
            ServerError::Forbidden => Forbidden(why),
            ServerError::BadType(_) => UnsupportedMediaType(why),
            ServerError::TooLarge => PayloadTooLarge(why),
            ServerError::RateLimited => TooManyRequests(why),
            ServerError::LegalReasons(_) => UnavailableForLegalReasons(why),
            ServerError::NotImplemented => NotImplemented(why),
            ServerError::Unavailable => ServiceUnavailable(why),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        Response::from(Error::from(self))
    }
}

impl From<RedisError> for ServerError {
    fn from(why: RedisError) -> Self {
        ServerError::RedisError(why)
    }
}

impl From<DbErr> for ServerError {
    fn from(db: DbErr) -> Self {
        match &db {
            DbErr::RecordNotFound(rec) => ServerError::NotFound(rec.to_owned(), ""),
            _ => ServerError::DatabaseError(db),
        }
    }
}

impl From<Error> for ServerError {
    fn from(why: Error) -> Self {
        return ServerError::InternalServer(why);
    }
}
