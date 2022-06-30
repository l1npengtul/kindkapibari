use crate::access::login::generate_login_token;
use crate::access::oauth_thirdparty::{
    get_oauth_client, get_user_data, AuthProviderDataCommon, OAuthAttempt,
};
use crate::State;
use axum::extract::Query;
use axum::{Extension, Json};
use chrono::Utc;
use kindkapibari_core::roles::Roles;
use kindkapibari_core::secret::SentSecret;
use kindkapibari_core::user_data::UserSignupRequest;
use kindkapibari_schema::access::user::{
    detect_user_already_exists, user_by_email, user_by_id, user_by_username,
};
use kindkapibari_schema::error::ServerError;
use kindkapibari_schema::redis::{
    check_if_exists_cache, delet_dis, insert_into_cache, read_from_cache,
};
use kindkapibari_schema::schema::users::{user, userdata};
use kindkapibari_schema::SResult;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, PkceCodeVerifier};
use redis::AsyncCommands;
use sea_orm::{ActiveValue, EntityTrait};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::Arc;
use tracing::instrument;
use utoipa::Component;

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize, Deserialize, Component)]
pub enum RedirectedUser {
    AlreadyExists(SentSecret),
    NewUserCreation {
        slip: String,
        suggested_username: Option<String>,
    },
}

#[derive(Deserialize, Component)]
pub struct StateAndCode {
    pub state: String,
    pub code: String,
}

#[instrument]
pub async fn redirect(
    Extension(app): Extension<Arc<State>>,
    state_and_code: Query<StateAndCode>,
) -> SResult<Json<RedirectedUser>> {
    let oauth_attempt = app
        .redis
        .get::<&String, OAuthAttempt>(&state_and_code.state)
        .await?;
    let config = *app.config.read().await;
    if &state_and_code.state != oauth_attempt.pkce_verifier() {
        return Err(ServerError::BadRequest(Cow::Borrowed("Bad State")));
    }

    let client_rebuild = match oauth_attempt.authorizer() {
        "twitter" => get_oauth_client(
            config.oauth.twitter.authorize_url,
            config.oauth.twitter.token_url,
            format!("{THIS_SITE_URL}/redirect"),
            config.oauth.twitter.client_id,
            config.oauth.twitter.secret,
        )?,
        "github" => get_oauth_client(
            config.oauth.github.authorize_url,
            config.oauth.github.token_url,
            format!("{THIS_SITE_URL}/redirect"),
            config.oauth.github.client_id,
            config.oauth.github.secret,
        )?,
        _ => return Err(ServerError::BadRequest(Cow::Borrowed("Bad Authorizer"))),
    };

    let token_result = client_rebuild
        .exchange_code(AuthorizationCode::new(state_and_code.code.clone()))
        .set_pkce_verifier(PkceCodeVerifier::new(
            oauth_attempt.pkce_verifier().to_string(),
        ))
        .request_async(async_http_client)
        .await
        .map_err(|why| ServerError::InternalServer(why))?;

    let user_info = get_user_data(oauth_attempt.authorizer(), token_result).await?;
    let maybe_existing_user = detect_user_already_exists(app.clone(), user_info).await?;
    let user_info_common: AuthProviderDataCommon = user_info.into();
    Ok(Json(match maybe_existing_user {
        Some(existing) => {
            RedirectedUser::AlreadyExists(generate_login_token(app.clone(), existing).await?)
        }
        None => {
            // in this case we create a "slip" that the user can trade for not making this request again
            if user_info_common.email.is_none() {
                return Err(ServerError::BadRequest(Cow::from("You need an email!")));
            }
            let user_info_num = format!(
                "{}{}{}{}",
                user_info_common.id,
                &user_info_common.email.unwrap_or_default(),
                &user_info_common.profile_picture,
                &user_info_common.username
            );
            // check if username exists
            let suggested = if !user_by_username(app.clone(), &user_info_num)
                .await?
                .is_none()
            {
                Some(user_info_common.username)
            } else {
                None
            };
            // we dont really need this to be cryptographic
            // by the time this is reversed it will already be useless
            let data_hashed = base64::encode(blake3::hash(user_info_num.as_bytes()).as_bytes());
            let stored_secret_with_redis_prefix =
                format!("{REDIS_USER_CREATION_PENDING_PREFIX}:{}", data_hashed);
            if !check_if_exists_cache(app.clone(), &stored_secret_with_redis_prefix).await {
                insert_into_cache(app.clone(), &data_hashed, &user_info_common, Some(1000)).await?;
            } else {
                return Err(ServerError::ISErr(Cow::from("failed to create slip")));
            }

            RedirectedUser::NewUserCreation {
                slip: data_hashed,
                suggested_username: suggested,
            }
        }
    }))
}

#[instrument]
#[utoipa::path(
    delete,
    path = "/burn_signup_token",
    responses(
    (status = 200, description = "Token Sucessfully burnt"),
    (status = 404, description = "No token exists"),
    (status = 500, description = "Failed")),
    params(
    ("request" = String, query, description = "token")
    )
)]
pub async fn burn_signup_token(
    Extension(state): Extension<Arc<State>>,
    request: Query<String>,
) -> SResult<()> {
    if !check_if_exists_cache(state.clone(), &request.0).await {
        return Err(ServerError::NotFound(
            Cow::from("signup in progress"),
            Cow::from(request.0),
        ));
    }
    delet_dis(state.clone(), &request.0).await?;
    Ok(())
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize, Component)]
pub struct PostSignupSent {
    pub id: u64,
    pub login_secret: SentSecret,
}

#[instrument]
#[utoipa::path(
    post,
    path = "/signup",
    responses(
    (status = 200, description = "Signup sucessfully completed", body = json),
    (status = 400, description = "Signup Request Modified/Invalid"),
    (status = 404, description = "Signup Request Does Not Exist"),
    (status = 500, description = "Failed")),
    params(
    ("data" = json, body, description = "UserSignupRequest Object")
    )
)]
pub async fn signup(
    Extension(state): Extension<Arc<State>>,
    request: Query<String>,
    data: Json<UserSignupRequest>,
) -> SResult<Json<PostSignupSent>> {
    let oauth_data = read_from_cache::<AuthProviderDataCommon>(state.clone(), &request.0).await?;
    let user_data = data.0;

    let user_id = state.id_generator.user_ids.generate_id();

    if user_by_id(state.clone(), user_id).await.is_ok() {
        return Err(ServerError::ISErr(Cow::from("please retry")));
    }

    if oauth_data.email.is_none()
        || user_data.email != oauth_data.email
        || user_by_email(state.clone(), &oauth_data.email)
            .await?
            .is_some()
    {
        return Err(ServerError::BadRequest(Cow::from("email")));
    }

    if user_by_username(state.clone(), &user_data.username)
        .await?
        .is_some()
    {
        return Err(ServerError::BadRequest(Cow::from("username")));
    }

    let user_active_model = user::ActiveModel {
        id: ActiveValue::Set(user_id),
        username: ActiveValue::Set(user_data.username),
        email: ActiveValue::Set(oauth_data.email.unwrap()),
        profile_picture: ActiveValue::Set(Some(oauth_data.profile_picture)),
        creation_date: ActiveValue::Set(Utc::now()),
        roles: ActiveValue::Set(vec![Roles::NormalUser].into()),
    };

    let user_data_active_model = userdata::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        gender: ActiveValue::Set(user_data.other_data.gender),
        pronouns: ActiveValue::Set(user_data.other_data.pronouns),
        birthday: ActiveValue::Set(user_data.other_data.birthday),
        locale: ActiveValue::Set(user_data.other_data.locale),
    };

    user::Entity::insert(user_active_model)
        .exec(&state.database)
        .await?;
    userdata::Entity::insert(user_data_active_model)
        .exec(&state.database)
        .await?;

    let login_generated = generate_login_token(state.clone(), user_id).await?;

    Ok(Json(PostSignupSent {
        id: user_id,
        login_secret: login_generated,
    }))
}
