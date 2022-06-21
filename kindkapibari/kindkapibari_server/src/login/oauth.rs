use crate::{scopes::make_kkbscope_scope, AppData, KKBScope, SResult, ServerError};
use kindkapibari_core::impl_redis;
use oxide_auth::{
    endpoint::{OwnerConsent, Solicitation},
    frontends::simple::endpoint::FnSolicitor,
};
use oxide_auth_poem::{request::OAuthRequest, response::OAuthResponse};
use poem::{
    http,
    web::{headers::ContentType, Data, Query},
    Response,
};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, str::FromStr, sync::Arc};
use tracing::instrument;

const REDIS_AUTHORIZE_LOGIN_REDIRECT_ID_HEADER: &'static str = "kkb:au_lg_rdr:";

#[derive(Serialize, Deserialize)]
struct AuthorizeRequest {
    client_id: String,
    client_secret: String,
    redirect_url: Option<String>,
    scopes: Vec<KKBScope>,
    state: String,
}

impl_redis!(AuthorizeRequest);

#[instrument]
#[handler]
pub async fn authorize_get(
    Data(state): Data<Arc<AppData>>,
    request: &OAuthRequest,
    scopes: Query<String>,
) -> SResult<OAuthResponse> {
    let scopes = make_kkbscope_scope(
        scopes
            .split(",")
            .map(|x| KKBScope::from_str(x))
            .collect::<Result<Vec<KKBScope>, ()>>()
            .map_err(|| ServerError::BadRequest(Cow::from("bad scopes")))?,
    );
    state
        .oauth
        .endpoint()
        .await
        .with_solicitor(FnSolicitor(consent_form))
        .with_scopes(scopes)
        .authorization_flow()
        .execute(request.clone())
        .map_err(|| ServerError::Unauthorized)
}

#[instrument]
#[handler]
pub async fn authorize_consent_post(
    Data(state): Data<Arc<AppData>>,
    request: &OAuthRequest,
    allow: poem::Result<Query<bool>>,
) -> SResult<OAuthResponse> {
    let allow = allow.unwrap_or(Query(false)).0;
    state
        .oauth
        .endpoint()
        .await
        .with_solicitor(FnSolicitor(move |_: &mut _, grant: Solicitation<'_>| {
            if allow {
                OwnerConsent::Authorized((&grant.pre_grant().client_id).clone())
            } else {
                OwnerConsent::Denied
            }
        }))
        .access_token_flow()
        .execute(request.clone())
        .map_err(|| ServerError::Forbidden)
}

#[instrument]
#[handler]
pub async fn token_post(
    Data(state): Data<Arc<AppData>>,
    request: OAuthRequest,
) -> SResult<OAuthResponse> {
    state
        .oauth
        .endpoint()
        .await
        .access_token_flow()
        .execute(request)
        .map_err(|| ServerError::Forbidden)
}

#[instrument]
#[handler]
pub async fn refresh_post(
    Data(state): Data<Arc<AppData>>,
    request: OAuthRequest,
) -> SResult<OAuthResponse> {
    state
        .oauth
        .endpoint()
        .await
        .refresh_flow()
        .execute(request)
        .map_err(|| ServerError::Forbidden)
}

fn consent_form(_: &mut OAuthRequest, solicitation: Solicitation) -> OwnerConsent<OAuthResponse> {
    OwnerConsent::InProgress(
        Response::builder()
            .status(http::StatusCode::OK)
            .typed_header::<ContentType>(ContentType::html())
            .body(consent_page_html("/authorize", solicitation))
            .into(),
    )
}

// taken from oxide-auth
fn consent_page_html(route: &str, solicitation: Solicitation) -> String {
    macro_rules! template {
        () => {
            "<html>'{0:}' (at {1:}) is requesting permission for '{2:}'
<form method=\"post\">
    <input type=\"submit\" value=\"Accept\" formaction=\"{4:}?{3:}&allow=true\">
    <input type=\"submit\" value=\"Deny\" formaction=\"{4:}?{3:}&deny=true\">
</form>
</html>"
        };
    }

    let grant = solicitation.pre_grant();
    let state = solicitation.state();

    let mut extra = vec![
        ("response_type", "code"),
        ("client_id", grant.client_id.as_str()),
        ("redirect_uri", grant.redirect_uri.as_str()),
    ];

    if let Some(state) = state {
        extra.push(("state", state));
    }

    format!(
        template!(),
        grant.client_id,
        grant.redirect_uri,
        grant.scope,
        serde_urlencoded::to_string(extra).unwrap(),
        &route,
    )
}
