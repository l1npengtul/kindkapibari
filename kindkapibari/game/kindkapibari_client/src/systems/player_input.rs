use crate::{action::Action, player::LocalPlayer};
use bevy::prelude::{Query, With};
use kindkapibari_game_shared::components::{
    entities::state::PlayerState, speed::Speed, velocity::Velocity,
};
use leafwing_input_manager::prelude::ActionState;

pub fn player_input(
    query: Query<&ActionState<Action>, With<(LocalPlayer, Speed, Velocity, PlayerState)>>,
) {
    // move player
}
