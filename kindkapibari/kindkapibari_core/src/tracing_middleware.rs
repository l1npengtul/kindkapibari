use poem::{Endpoint, IntoResponse, Middleware, Request, Response};
use tokio::time::Instant;

#[derive(Default)]
pub struct TracingMiddleware;

impl<E: Endpoint> Middleware<E> for TracingMiddleware {
    type Output = TracingMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        TracingMiddlewareImpl { endpoint: ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct TracingMiddlewareImpl<E> {
    endpoint: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for TracingMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        let user_agent = req
            .headers()
            .get("user-agent")
            .and_then(|s| s.to_str().ok())
            .unwrap_or("");

        let host = req
            .headers()
            .get("host")
            .and_then(|s| s.to_str().ok())
            .unwrap_or("");

        let user_agent_mobile = req
            .headers()
            .get("sec-ch-ua-mobile")
            .and_then(|s| s.to_str().ok())
            .unwrap_or("");

        let user_agent_platform = req
            .headers()
            .get("sec-ch-ua-platform")
            .and_then(|s| s.to_str().ok())
            .unwrap_or("");

        let span = info_span!(
            "http request",
            otel.name = %req.uri().path(),
            otel.kind = "server",
            http.method = %req.method(),
            http.url = %req.uri(),
            http.status_code = field::Empty,
            http.user_agent = &user_agent,
            http.host = &host,
            http.sec_ch_ua_mobile = &user_agent_mobile,
            http.sec_ch_ua_platform = &user_agent_platform,
            http.remote_addr = %req.remote_addr,
            request_id = field::Empty,
            user_id = field::Empty,
        );

        async move {
            let now = Instant::now();
            let res = self.inner.call(req).await;
            let duration = now.elapsed();

            match res {
                Ok(resp) => {
                    let resp = resp.into_response();
                    tracing::info!(
                        status = %resp.status(),
                        duration = ?duration,
                        "response"
                    );
                    Ok(resp)
                }
                Err(err) => {
                    tracing::info!(
                        error = %err,
                        duration = ?duration,
                        "error"
                    );
                    Err(err)
                }
            }
        }
        .instrument(span)
        .await
    }
}
