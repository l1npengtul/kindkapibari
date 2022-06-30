use async_trait::async_trait;
use axum::{headers::Cookie, TypedHeader};
use axum_core::body::boxed;
use axum_core::{
    extract::{FromRequest, RequestParts},
    response::{IntoResponse, Response},
};
use http::StatusCode;
use http_body::Empty;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub enum Located {
    Query(Cow<'static, str>),
    Header(Cow<'static, str>),
    Cookie(Cow<'static, str>),
}

#[async_trait]
pub trait FromAuth: Sized {
    const LOCATION: Located;

    async fn from_auth(provided: String) -> Option<Self>;
}

pub enum AuthenticationRejection {
    BadQuery,
    BadHeader,
    BadCookie,
    BadAuthorization,
}

impl IntoResponse for AuthenticationRejection {
    fn into_response(self) -> Response {
        let statuscode = match self {
            AuthenticationRejection::BadQuery => StatusCode::UNPROCESSABLE_ENTITY,
            AuthenticationRejection::BadHeader => StatusCode::UNPROCESSABLE_ENTITY,
            AuthenticationRejection::BadCookie => StatusCode::UNPROCESSABLE_ENTITY,
            AuthenticationRejection::BadAuthorization => StatusCode::UNAUTHORIZED,
        };

        Response::builder()
            .status(statuscode)
            .body(boxed(Empty::new()))
            .unwrap()
            .into()
    }
}

pub struct Authentication<T>(pub T)
where
    T: DeserializeOwned + FromAuth;

#[async_trait]
impl<T, B> FromRequest<B> for Authentication<T>
where
    T: DeserializeOwned + FromAuth,
    B: Send,
{
    type Rejection = AuthenticationRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let data = match T::LOCATION {
            Located::Query(_query_var) => {
                let query = req.uri().query().unwrap_or_default();
                let value = serde_urlencoded::from_str::<String>(query)
                    .map_err(|_| AuthenticationRejection::BadQuery)?;
                T::from_auth(value)
                    .await
                    .ok_or(AuthenticationRejection::BadAuthorization)?
            }
            Located::Header(header_key) => {
                let header_value = req
                    .headers()
                    .get(header_key.as_ref())
                    .ok_or(AuthenticationRejection::BadHeader)?
                    .to_str()
                    .map_err(|_| AuthenticationRejection::BadHeader)?
                    .to_string();
                T::from_auth(header_value)
                    .await
                    .ok_or(AuthenticationRejection::BadAuthorization)?
            }
            Located::Cookie(cookie_key) => {
                let cookie_value = Option::<TypedHeader<Cookie>>::from_request(req)
                    .await
                    .map_err(|_| AuthenticationRejection::BadHeader)?
                    .map(|cookie| cookie.get(cookie_key.as_ref()))
                    .flatten()
                    .ok_or(AuthenticationRejection::BadCookie)?;
                T::from_auth(cookie_value.to_string())
                    .await
                    .ok_or(AuthenticationRejection::BadAuthorization)?
            }
        };

        Ok(Authentication(data))
    }
}
