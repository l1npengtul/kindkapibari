use axum::async_trait;
use axum::http::Request;
use axum::response::Response;
use futures::future::BoxFuture;
use hyper::Body;
use kindkapibari_core::state::State;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

pub struct AllowedLayer<P, U, D>
where
    P: Permissions,
    U: User,
    D: State,
{
    pub permission: P,
    pub state: D,
}

impl<U, P, S, D> Layer<S> for AllowedLayer<P, U, D>
where
    P: Permissions,
    U: User,
    D: State,
{
    type Service = IsAllowed<U, P, S, D>;

    fn layer(&self, inner: S) -> Self::Service {
        IsAllowed {
            inner,
            permission: self.permission.clone(),
            state: self.state.clone(),
        }
    }
}

pub struct IsAllowed<U, P, S, D>
where
    P: Permissions,
    U: User,
    D: State,
{
    inner: S,
    permission: P,
    state: D,
}

impl<U, P, S, D> Service<S> for IsAllowed<U, P, S, D>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
    P: Permissions,
    U: User,
    D: State,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: S) -> Self::Future {
        let authentication = req.
    }
}

pub trait Permissions: PartialEq + Display + Sized + Clone {
    fn all() -> Vec<Self>;
    fn anonymous() -> Self;
    fn admin() -> Self;
    fn has_permission(&self, incoming: &[Self]) -> bool {
        let mut has = false;
        if self == Self::admin() {
            return true;
        } else {
            for perm in incoming {
                if perm == self {
                    has = true;
                    break;
                }
            }
        }
        has
    }
}

#[async_trait]
pub trait User: Sized {
    async fn get_user_by_authorization<S: State>(state: Arc<S>, auth: String) -> Self;
    fn permissions<P: Permissions>(&self) -> Vec<P>;
}

pub struct AuthorizedUser<U, P>
where
    U: User,
    P: Permissions,
{
    pub user: U,
    pub permissions: Vec<P>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Scopes {
    Admin,
    Report,
    ListReports,
    Yank,
    Download,
    Publish,
}
