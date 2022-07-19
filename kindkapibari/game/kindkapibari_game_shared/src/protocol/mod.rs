pub mod auth;
pub mod connection;
pub mod entity;
mod input;
pub mod map;
pub mod total;
pub mod transform;

use crate::protocol::{
    auth::Auth,
    connection::ClientConnectionStateUpdate,
    entity::EntityUpdate,
    input::{ActiveWeaponChange, Attack, LookDirection, PlayerMove},
    map::{LoadMapUpdate, MapBuildUpdated},
    total::TotalGameStateUpdate,
    transform::TransformUpdate,
};
use naia_shared::Protocolize;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Protocolize)]
#[non_exhaustive]
pub enum Protocol {
    // Client 2 Server
    ClientAuth(Auth),
    ClientInputMovement(PlayerMove),
    ClientInputJump,
    ClientInputChangeActive(ActiveWeaponChange),
    ClientInputAttack(Attack),
    ClientInputMoveCameraRotate(LookDirection),
    // Server 2 Client
    ConnectionStateUpdate(ClientConnectionStateUpdate),
    TotalGameStateUpdate(TotalGameStateUpdate),
    TransformUpdate(TransformUpdate),
    EntityAssignment(EntityUpdate),
    EntityStateUpdate(EntityUpdate),
    LoadMapUpdate(LoadMapUpdate),
    MapBuildUpdated(MapBuildUpdated),
}
