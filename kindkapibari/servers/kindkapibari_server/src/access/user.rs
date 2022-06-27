use crate::access::auth::oauth_thirdparty::AuthorizationProviders;
use crate::schema::users::connections;
use crate::users::{sobers, userdata};
use crate::{user, AppData, SResult, ServerError};
use kindkapibari_core::sober::{Sober, Sobers};
use kindkapibari_core::user_data::UserData;
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::Arc;
use tracing::instrument;

#[instrument]
pub async fn user_by_id(state: Arc<AppData>, id: u64) -> SResult<user::Model> {
    // check our local cache
    if let Some(possible_user) = state.caches.users.get(&id) {
        return possible_user.ok_or(ServerError::NotFound(
            Cow::from("user"),
            Cow::from(id.to_string()),
        ));
    }

    let user = user::Entity::find_by_id(id).one(&state.database).await?;
    state.caches.users.insert(id, user.clone()).await;
    return user.ok_or(ServerError::NotFound(
        Cow::from("user"),
        Cow::from(id.to_string()),
    ));
}

#[instrument]
pub async fn user_by_username(
    state: Arc<AppData>,
    name: impl AsRef<str>,
) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(name.as_ref()))
        .one(&state.database)
        .await?;
    Ok(user)
}

#[instrument]
pub async fn user_by_email(
    state: Arc<AppData>,
    email: impl AsRef<str>,
) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email.as_ref()))
        .one(&state.database)
        .await?;
    Ok(user)
}

#[instrument]
pub async fn detect_user_already_exists(
    state: Arc<AppData>,
    maybeuser: AuthorizationProviders,
) -> SResult<Option<u64>> {
    // check if the account exists in connections
    let prepared = connections::Entity::find();

    let exists = match &maybeuser {
        AuthorizationProviders::Twitter(twt) => {
            prepared
                .filter(connections::Column::TwitterId.eq(Some(&twt.twitter_id)))
                .one(&state.database)
                .await?
        }
        AuthorizationProviders::Github(ghb) => {
            prepared
                .filter(connections::Column::GithubId.eq(Some(ghb.github_id as u64)))
                .one(&state.database)
                .await?
        }
    };

    if let Some(user) = exists {
        return Ok(Some(user.user_id));
    }

    // email account
    let email_chk = match maybeuser {
        AuthorizationProviders::Twitter(twt) => twt.email,
        AuthorizationProviders::Github(ghb) => ghb.email,
    };

    if let Some(email) = email_chk {
        return match user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&state.database)
            .await?
        {
            Some(user) => Ok(Some(user.id)),
            None => Ok(None),
        };
    }

    return Ok(None);
}

#[instrument]
pub async fn user_data_by_user_id(state: Arc<AppData>, user: u64) -> SResult<UserData> {
    let userdata = user::Entity::find_by_id(user)
        .join(JoinType::LeftJoin, userdata::Relation::User.def())
        .into_model::<userdata::Model>()
        .one(&state.database)
        .await?
        .ok_or(ServerError::NotFound(
            Cow::from("userdata"),
            Cow::from(user.to_string()),
        ))?;
    Ok(userdata.into_userdata())
}

#[instrument]
pub async fn update_user_data_by_user_id(
    state: Arc<AppData>,
    user: u64,
    userdata: UserData,
) -> SResult<()> {
    let user = user_by_id(state.clone(), user).await?;
    let mut user_data_active: userdata::ActiveModel = user.into();
    user_data_active.locale = ActiveValue::Set(userdata.locale);
    user_data_active.birthday = ActiveValue::Set(userdata.birthday);
    user_data_active.gender = ActiveValue::Set(userdata.gender);
    user_data_active.pronouns = ActiveValue::Set(userdata.pronouns);
    user_data_active.update(&state.database).await?;
    Ok(())
}

#[instrument]
pub async fn get_sobers_by_user_id(state: Arc<AppData>, user: u64) -> SResult<Sobers> {
    let user = user_by_id(state.clone(), user).await?;
    let sobers: Vec<sobers::Model> = user
        .find_related(sobers::Entity)
        .all(&state.database)
        .await?;
    let sobers = sobers
        .into_iter()
        .map(|sober_mdl| Sober {
            name: sober_mdl.name,
            start_time: sober_mdl.time_since_reset,
        })
        .collect::<Vec<Sober>>();
    Ok(Sobers { sobers })
}
