use bevy::prelude::{KeyCode, MouseButton};
use leafwing_input_manager::{input_map::InputMap, Actionlike};
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize, Actionlike,
)]
pub enum Action {
    AttackPrimary,
    AttackSecondary,
    AttackActivate,
    MoveForward,
    MoveLeft,
    MoveBack,
    MoveRight,
    Run,
    Crouch,
    Jump,
    Reload,
    Interact,
    Inventory,
    WeaponPrimary,
    WeaponSecondary,
    WeaponSwap,
    Options,
    ChatGlobal,
    ChatGuild,
    ChatParty,
    Kill, // the funny kill bind from the epic bideo game treeam fortress 2 where a team defen ds a fortress
}

pub fn default_kb_map() -> InputMap<Action> {
    InputMap::new([
        (MouseButton::Left, Action::AttackPrimary),
        (MouseButton::Right, Action::AttackSecondary),
        (KeyCode::C, Action::AttackActivate),
        (KeyCode::W, Action::MoveForward),
        (KeyCode::A, Action::MoveLeft),
        (KeyCode::S, Action::MoveBack),
        (KeyCode::D, Action::MoveRight),
        (KeyCode::LShift, Action::Run),
        (KeyCode::LControl, Action::Crouch),
        (KeyCode::Space, Action::Jump),
        (KeyCode::R, Action::Reload),
        (KeyCode::E, Action::Interact),
        (KeyCode::I, Action::Inventory),
        (KeyCode::Key1, Action::WeaponPrimary),
        (KeyCode::Key2, Action::WeaponSecondary),
        (KeyCode::Q, Action::WeaponSwap),
        (KeyCode::Escape, Action::Options),
        (KeyCode::Y, Action::ChatGlobal),
        (KeyCode::U, Action::ChatGuild),
        (KeyCode::P, Action::ChatParty),
        (KeyCode::Home, Action::Kill),
    ])
}
