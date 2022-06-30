use crate::{
    access::auth::oauth_thirdparty::AuthorizationProviders,
    appdata_traits::{AppDataCache, AppDataDatabase},
    schema::users::{connections, onetime_reminders, sobers, user, userdata},
    SResult, ServerError,
};
use chrono::{Duration, Utc};
use kindkapibari_core::{
    reminder::{OneTimeReminder, OneTimeReminders, Reminders},
    sober::{Sober, Sobers},
    user_data::UserData,
};
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, QueryFilter, QuerySelect,
    RelationTrait,
};
use std::{borrow::Cow, sync::Arc};
use tracing::instrument;

pub async fn user_by_id(
    state: Arc<impl AppDataDatabase + AppDataCache<u64, user::Model>>,
    id: u64,
) -> SResult<user::Model> {
    // check our local cache
    if let Some(possible_user) = state.cache::<u64, user::Model>().get(&id) {
        return possible_user.ok_or(ServerError::NotFound(
            Cow::from("user"),
            Cow::from(id.to_string()),
        ));
    }

    let user = match user::Entity::find_by_id(id).one(state.database()).await? {
        Some(u) => {
            state
                .cache::<u64, user::Model>()
                .insert(id, u.clone())
                .await;
            u
        }
        None => return Err(ServerError::NotFound(Cow::from("user"), Cow::from("id"))),
    };

    return user.ok_or(ServerError::NotFound(
        Cow::from("user"),
        Cow::from(id.to_string()),
    ));
}

#[instrument]
pub async fn user_by_username(
    state: Arc<impl AppDataDatabase>,
    name: impl AsRef<str>,
) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(name.as_ref()))
        .one(state.database())
        .await?;
    Ok(user)
}

#[instrument]
pub async fn user_by_email(
    state: Arc<impl AppDataDatabase>,
    email: impl AsRef<str>,
) -> SResult<Option<user::Model>> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email.as_ref()))
        .one(state.database())
        .await?;
    Ok(user)
}

#[instrument]
pub async fn detect_user_already_exists(
    state: Arc<impl AppDataDatabase>,
    maybeuser: AuthorizationProviders,
) -> SResult<Option<u64>> {
    // check if the account exists in connections
    let prepared = connections::Entity::find();

    let exists = match &maybeuser {
        AuthorizationProviders::Twitter(twt) => {
            prepared
                .filter(connections::Column::TwitterId.eq(Some(&twt.twitter_id)))
                .one(state.database())
                .await?
        }
        AuthorizationProviders::Github(ghb) => {
            prepared
                .filter(connections::Column::GithubId.eq(Some(ghb.github_id as u64)))
                .one(state.database())
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
            .one(state.database())
            .await?
        {
            Some(user) => Ok(Some(user.id)),
            None => Ok(None),
        };
    }

    return Ok(None);
}

#[instrument]
pub async fn user_data_by_user_id(
    state: Arc<impl AppDataDatabase>,
    user: u64,
) -> SResult<UserData> {
    let userdata = user::Entity::find_by_id(user)
        .join(JoinType::LeftJoin, userdata::Relation::User.def())
        .into_model::<userdata::Model>()
        .one(state.database())
        .await?
        .ok_or(ServerError::NotFound(
            Cow::from("userdata"),
            Cow::from(user.to_string()),
        ))?;
    Ok(userdata.into_userdata())
}

#[instrument]
pub async fn update_user_data_by_user_id(
    state: Arc<impl AppDataDatabase>,
    user: u64,
    userdata: UserData,
) -> SResult<()> {
    let user = user_by_id(state.clone(), user).await?;
    let mut user_data_active: userdata::ActiveModel = user.into();
    user_data_active.locale = ActiveValue::Set(userdata.locale);
    user_data_active.birthday = ActiveValue::Set(userdata.birthday);
    user_data_active.gender = ActiveValue::Set(userdata.gender);
    user_data_active.pronouns = ActiveValue::Set(userdata.pronouns);
    user_data_active.update(state.database()).await?;
    Ok(())
}

#[instrument]
pub async fn get_sobers_by_user_id(state: Arc<impl AppDataDatabase>, user: u64) -> SResult<Sobers> {
    let user = user_by_id(state.clone(), user).await?;
    let sobers: Vec<sobers::Model> = user
        .find_related(sobers::Entity)
        .all(state.database())
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

#[instrument]
async fn check_if_sober_already_exists(
    state: Arc<impl AppDataDatabase>,
    sober_name: &String,
    user: u64,
) -> SResult<bool> {
    let sobers = get_sobers_by_user_id(state.clone(), user)
        .await?
        .sobers
        .into_iter()
        .map(|sober| sober.name)
        .collect::<Vec<String>>();
    if sobers.contains(sober_name) {
        return Ok(true);
    }
    return Ok(false);
}

#[instrument]
pub async fn reset_sober_by_name_and_user_id(
    state: Arc<impl AppDataDatabase + AppDataCache<u64, user::Model>>,
    sober_name: String,
    user: u64,
) -> SResult<i64> {
    let user = user_by_id(state.clone(), user).await?;
    let sobers: Option<sobers::Model> = user
        .find_related(sobers::Entity)
        .filter(sobers::Column::Name.eq(sober_name.clone()))
        .one(state.database())
        .await?;

    let timestamp = match sobers {
        Some(sober) => {
            let mut sober_active_mdl = sober.into_active_model();
            let new_time = Utc::now();
            sober_active_mdl.time_since_reset = ActiveValue::Set(new_time.clone());
            sober_active_mdl.update(state.database()).await?;
            new_time.timestamp_millis()
        }
        None => {
            return Err(ServerError::NotFound(
                Cow::from("sober"),
                Cow::from(sober_name),
            ))
        }
    };

    Ok(timestamp)
}

#[instrument]
pub async fn update_sober_name_by_user_id(
    state: Arc<impl AppDataDatabase>,
    current: String,
    new_name: String,
    user_id: u64,
) -> SResult<()> {
    let user = user_by_id(state.clone(), user_id).await?;
    if !check_if_sober_already_exists(state.clone(), &new_name, user_id).await? {
        return Err(ServerError::BadRequest(Cow::from("already exists!")));
    }
    let sobers: Option<sobers::Model> = user
        .find_related(sobers::Entity)
        .filter(sobers::Column::Name.eq(current.clone()))
        .one(state.database())
        .await?;

    match sobers {
        Some(sober) => {
            let mut sober_active_mdl = sober.into_active_model();
            sober_active_mdl.name = ActiveValue::Set(new_name);
            sober_active_mdl.update(state.database()).await?;
        }
        None => {
            return Err(ServerError::NotFound(
                Cow::from("sober"),
                Cow::from(current),
            ))
        }
    };

    Ok(())
}

#[instrument]
pub async fn add_sober_by_user(
    state: Arc<impl AppDataDatabase>,
    user: u64,
    new_sober: Sober,
) -> SResult<()> {
    let _ = user_by_id(state.clone(), user).await?;
    let now = Utc::now();
    if now - new_sober.start_time > Duration::seconds(5) || new_sober.start_time > now {
        return Err(ServerError::BadRequest(Cow::from("bad time")));
    }
    if !check_if_sober_already_exists(state.clone(), &new_sober.name, user_id).await? {
        return Err(ServerError::BadRequest(Cow::from("already exists!")));
    }
    let sober_active = sobers::ActiveModel {
        id: Default::default(),
        owner: ActiveValue::Set(user),
        name: ActiveValue::Set(new_sober.name),
        time_since_reset: ActiveValue::Set(new_sober.start_time),
    };

    sober_active.insert(state.database()).await?;
    Ok(())
}

#[instrument]
pub async fn delete_sober_name_by_user_id(
    state: Arc<impl AppDataDatabase>,
    user: u64,
    sober_name: String,
) -> SResult<()> {
    let user = user_by_id(state.clone(), user).await?;
    let sobers: Option<sobers::Model> = user
        .find_related(sobers::Entity)
        .filter(sobers::Column::Name.eq(sober_name.clone()))
        .one(state.database())
        .await?;

    match sobers {
        Some(sober) => {
            sober.delete(state.database()).await?;
        }
        None => {
            return Err(ServerError::NotFound(
                Cow::from("sober"),
                Cow::from(sober_name),
            ))
        }
    };

    Ok(())
}

#[instrument]
async fn check_if_onetime_already_exists(
    state: Arc<impl AppDataDatabase>,
    user: u64,
    new_name: &String,
) -> SResult<bool> {
    let one_time = get_onetime_reminders_by_user_id(state.clone(), user)
        .await?
        .one_time
        .into_iter()
        .map(|one_time| one_time.name)
        .collect::<Vec<String>>();
    if one_time.contains(new_name) {
        return Ok(true);
    }
    return Ok(false);
}

#[instrument]
pub async fn get_onetime_reminders_by_user_id(
    state: Arc<impl AppDataDatabase>,
    user: u64,
) -> SResult<OneTimeReminders> {
    let user = user_by_id(state.clone(), user).await?;
    let onetimes: Vec<onetime_reminders::Model> = user
        .find_related(onetime_reminders::Entity)
        .all(state.database())
        .await?;
    let onetimes = onetimes
        .into_iter()
        .map(|reminders| OneTimeReminder {
            name: reminders.name,
            set: reminders.set,
            expire: reminders.expire,
        })
        .collect::<Vec<OneTimeReminder>>();
    Ok(OneTimeReminders { one_time: onetimes })
}

#[instrument]
pub async fn expire_onetime_reminder_by_user_id(
    state: Arc<impl AppDataDatabase + AppDataCache<u64, user::Model>>,
    user: u64,
    name: String,
) -> SResult<()> {
    let user = user_by_id(state.clone(), user).await?;
}
