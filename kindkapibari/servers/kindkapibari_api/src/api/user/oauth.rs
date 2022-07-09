// use crate::{KKBScope, SResult, State};
// use axum::extract::Query;
// use axum::response::Response;
// use axum::{http, Extension};
// use kindkapibari_core::impl_redis;
// use kindkapibari_core::scopes::make_kkbscope_scope;
// use kindkapibari_schema::error::ServerError;
// use oxide_auth::{
//     endpoint::{OwnerConsent, Solicitation},
//     frontends::simple::endpoint::FnSolicitor,
// };
// use oxide_auth_axum::{OAuthRequest, OAuthResponse};
// use poem_openapi::{Object, OpenApi, OpenApiService};
// use serde::{Deserialize, Serialize};
// use std::{borrow::Cow, str::FromStr, sync::Arc};
// use tracing::instrument;
// use utoipa::openapi::Object;
// use kindkapibari_schema::SResult;
//
// const REDIS_AUTHORIZE_LOGIN_REDIRECT_ID_HEADER: &'static str = "kkb:au_lg_rdr:";
//
// #[derive(Object, Serialize, Deserialize)]
// pub struct AuthorizeRequest {
//     client_id: String,
//     client_secret: String,
//     redirect_url: Option<String>,
//     scopes: Vec<KKBScope>,
//     state: String,
// }
//
// impl_redis!(AuthorizeRequest);
//
// #[instrument]
// pub async fn authorize_get(
//     Extension(state): Extension<Arc<State>>,
//     request: &OAuthRequest,
//     scopes: Query<String>,
// ) -> SResult<OAuthResponse> {
//     let scopes = make_kkbscope_scope(
//         scopes
//             .split(",")
//             .map(|x| KKBScope::from_str(x))
//             .collect::<Result<Vec<KKBScope>, ()>>()
//             .map_err(|| ServerError::BadRequest(Cow::from("bad scopes")))?,
//     );
//     state
//         .oauth
//         .endpoint()
//         .await
//         .with_solicitor(FnSolicitor(consent_form))
//         .with_scopes(scopes)
//         .authorization_flow()
//         .execute(request.clone())
//         .map_err(|| ServerError::Unauthorized)
// }
//
// #[instrument]
// pub async fn authorize_consent_post(
//     Extension(state): Extension<Arc<State>>,
//     request: &OAuthRequest,
//     allow: Query<bool>,
// ) -> SResult<OAuthResponse> {
//     let allow = allow.unwrap_or(Query(false)).0;
//     state
//         .oauth
//         .endpoint()
//         .await
//         .with_solicitor(FnSolicitor(move |_: &mut _, grant: Solicitation<'_>| {
//             if allow {
//                 OwnerConsent::Authorized((&grant.pre_grant().client_id).clone())
//             } else {
//                 OwnerConsent::Denied
//             }
//         }))
//         .access_token_flow()
//         .execute(request.clone())
//         .map_err(|| ServerError::Forbidden)
// }
//
// #[instrument]
// pub async fn token_post(
//     Extension(state): Extension<Arc<State>>,
//     request: OAuthRequest,
// ) -> SResult<OAuthResponse> {
//     state
//         .oauth
//         .endpoint()
//         .await
//         .access_token_flow()
//         .execute(request)
//         .map_err(|| ServerError::Forbidden)
// }
//
// #[instrument]
// pub async fn refresh_post(
//     Extension(state): Extension<Arc<State>>,
//     request: OAuthRequest,
// ) -> SResult<OAuthResponse> {
//     state
//         .oauth
//         .endpoint()
//         .await
//         .refresh_flow()
//         .execute(request)
//         .map_err(|| ServerError::Forbidden)
// }
//
// fn consent_form(_: &mut OAuthRequest, solicitation: Solicitation) -> OwnerConsent<OAuthResponse> {
//     OwnerConsent::InProgress(
//         Response::builder()
//             .status(http::StatusCode::OK)
//             .header("Content-Type", "text/html")
//             .body(consent_page_html("/authorize", solicitation))
//             .into(),
//     )
// }
//
