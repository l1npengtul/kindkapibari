use bevy::prelude::KeyCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Actions {
    AttackPrimary,
    AttackSecondary,
    AttackActivate,
    AttackThird,
    AttackThirdEmulateKey,
    MoveForward,
    MoveLeft,
    MoveBack,
    MoveRight,
    Reload,
    Interact,
    Inventory,
    WeaponPrimary,
    WeaponSecondary,
    WeaponThird,
    Options,
    ChatGlobal,
    ChatGuild,
    ChatParty,
}
