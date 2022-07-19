use bevy::math::Quat;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveDirection {
    W,
    WA,
    WD,
    A,
    S,
    SA,
    SD,
    D,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveSpeedModifier {
    Normal,
    Run,
    Crouched,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayerMove {
    pub direction: Option<MoveDirection>,
    pub modifier: MoveSpeedModifier,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActiveWeaponChange {
    Primary,
    Secondary,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Attack {
    Primary,
    Secondary,
    Activate,
    Reload,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct LookDirection {
    pub camera_facing: Quat,
    pub player_facing: Quat,
}
