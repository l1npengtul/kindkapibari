use crate::schema::{coconutpak, subscribers, user};
use crate::{AppData, SResult};
use sea_orm::{
    ActiveValue, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use std::sync::Arc;
use tracing::instrument;
use tracing::log::{error, log, warn};

#[instrument]
pub async fn get_user_subscribes(
    state: Arc<AppData>,
    user_id: u64,
) -> SResult<Vec<coconutpak::Model>> {
    log!(
        "get_user_subscribes: ",
        user = %user_id,
    );
    let subscribed_paks = user::Entity::find_by_id(user_id)
        .join(JoinType::RightJoin, subscribers::Relation::User.def())
        .join(JoinType::RightJoin, subscribers::Relation::CoconutPak.def())
        .into_model::<coconutpak::Model>()
        .all(&state.database)
        .await?;

    Ok(subscribed_paks)
}

#[instrument]
pub async fn post_user_subscribes(state: Arc<AppData>, user_id: u64, pak_id: u64) -> SResult<()> {
    let active_subscribe = subscribers::ActiveModel {
        user_id: ActiveValue::Set(user_id),
        pak_id: ActiveValue::Set(pak_id),
    };

    active_subscribe.insert(&state.database).await?;
    Ok(())
}

#[instrument]
pub async fn get_user_coconut_paks(
    state: Arc<AppData>,
    user_id: u64,
) -> SResult<Vec<coconutpak::Model>> {
    let paks = coconutpak::Entity::find()
        .filter(coconutpak::Column::Owner.eq(user_id))
        .all(&state.database)
        .await?;
    Ok(paks)
}
