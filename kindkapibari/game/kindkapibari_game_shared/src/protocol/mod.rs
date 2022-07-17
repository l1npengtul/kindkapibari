pub mod auth;
pub mod entity;
pub mod map;
pub mod total;
pub mod transform;

use crate::protocol::{
    auth::Auth,
    entity::EntityUpdate,
    map::{LoadMapUpdate, MapBuildUpdated},
    total::TotalGameStateUpdate,
    transform::TransformUpdate,
};
use naia_shared::Protocolize;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Protocolize)]
#[non_exhaustive]
pub enum Protocol {
    ClientAuth(Auth),
    ClientConnection(bool),
    TotalGameStateUpdate(TotalGameStateUpdate),
    TransformUpdate(TransformUpdate),
    EntityAssignment(EntityUpdate),
    EntityStateUpdate(EntityUpdate),
    LoadMapUpdate(LoadMapUpdate),
    MapBuildUpdated(MapBuildUpdated),
}
