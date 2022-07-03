// use crate::{
//     appdata_traits::{AppDataCache, AppDataDatabase, AppDataRedis},
//     schema::applications,
//     SResult, ServerError,
// };
// use redis::{AsyncCommands, ControlFlow, Msg};
// use sea_orm::{ActiveValue, EntityTrait};
// use std::{fmt::Display, sync::Arc};
// use tracing::instrument;

// #[instrument]
// pub async fn application_by_id(
//     state: Arc<impl AppDataDatabase + AppDataCache<u64, applications::Model>>,
//     id: u64,
// ) -> SResult<applications::Model> {
//     // from_cache
//     if let Some(app) = state.cache::<u64, applications::Model>().get(&id) {
//         return app.ok_or(ServerError::NotFound(
//             format!("{id}").into(),
//             "Not Found".into(),
//         ));
//     }
//
//     let application_query = match applications::Entity::find_by_id(id)
//         .one(state.database())
//         .await?
//     {
//         Some(app) => {
//             state
//                 .cache::<u64, applications::Model>()
//                 .insert(id, app.clone()); // rip alloc
//             app
//         }
//         None => ServerError::NotFound("application".into(), format!("{id}").into()),
//     };
//     // commit to cache
//
//     application_query.ok_or(ServerError::NotFound(
//         format!("{id}").into(),
//         "Not Found".into(),
//     ))
// }
//
// pub fn invalidate_application_cache(
//     state: Arc<impl AppDataRedis + AppDataCache<u64, applications::Model>>,
//     msg: Msg,
// ) -> ControlFlow<()> {
//     if let Ok(id) = msg.get_pattern::<u64>() {
//         state
//             .cache::<u64, applications::Model>()
//             .blocking_invalidate(id);
//     }
//     ControlFlow::Continue
// }
//
// #[instrument]
// pub async fn new_application(
//     state: Arc<impl AppDataRedis + AppDataDatabase + AppDataCache<u64, applications::Model>>, // impl L + Ratio + AppData + UrBad + Studie
//     application: applications::ActiveModel,
// ) -> SResult<u64> {
//     let mut application = application;
//     let new_id = state.id_generator.generate_id();
//     application.id = ActiveValue::Set(new_id);
//     application.insert(state.database()).await?;
//     state.redis().publish("APPLICATION_CACHE", new_id);
//     Ok(new_id)
// }
