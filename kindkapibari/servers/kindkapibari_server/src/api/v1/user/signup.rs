use crate::{
    access::{
        auth::{login::generate_login_token, oauth_thirdparty::AuthProviderDataCommon},
        check_if_exists_cache, delet_dis, read_from_cache,
        user::{user_by_email, user_by_id, user_by_username},
    },
    roles::Roles,
    user,
    users::userdata,
    AppData, SResult, ServerError,
};
use axum::{extract::Query, Extension, Json};
use chrono::Utc;
use kindkapibari_core::user_data::{PostSignupSent, UserSignupRequest};
use sea_orm::{ActiveValue, EntityTrait};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

#[instrument]
pub async fn burn_signup_token(
    Extension(state): Extension<Arc<AppData>>,
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

#[instrument]
pub async fn signup(
    Extension(state): Extension<Arc<AppData>>,
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
