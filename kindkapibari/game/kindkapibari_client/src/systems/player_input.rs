use crate::{action::Action, player::LocalPlayer};
use bevy::{
    math::{UVec2, UVec3, Vec2, Vec3},
    prelude::{Query, With},
};
use kindkapibari_game_shared::components::{
    entities::state::PlayerState, speed::Speed, velocity::Velocity,
};
use leafwing_input_manager::prelude::ActionState;

pub fn player_keyboaord_input(
    mut query: Query<
        (
            &ActionState<Action>,
            &Speed,
            &mut Velocity,
            &mut PlayerState,
        ),
        With<LocalPlayer>,
    >,
) {
    // get pressed key
    let (action_state, speed, velocity, state) = query.single_mut();

    // Bevy's movement works like minecraft, where Z is up.
    let mut input_vector = Vec3::ZERO;
    // The else statements are for null-movement cancellation. People shouldn't randomly stop because they pressed the 2 wrong keys! (i am a compet player)
    if action_state.just_pressed(Action::MoveForward) {
        input_vector.x += 1_f32
    } else if action_state.just_pressed(Action::MoveBack) {
        input_vector.x -= 1_f32
    }
    if action_state.just_pressed(Action::MoveLeft) {
        input_vector.z += 1_f32
    } else if action_state.just_pressed(Action::MoveRight) {
        input_vector.z -= 1_f32
    }
    input_vector = input_vector.normalize();

    let mut move_speed = speed.max_speed as f32;
    if action_state.just_pressed(Action::Run) {
        move_speed *= speed.run_multi;
    } else if action_state.just_pressed(Action::Crouch) {
        move_speed *= speed.crouch_multi;
    }

    if input_vector == Vec3::ZERO {
        velocity.vel
    }
}

pub fn player_mouse_input() {}

fn move_towards_2d(target: Vec3, current: Vec3, step: f32) -> Vec3 {
    let target_v2 = Vec2::new(target.x, target.z);
    let current_v2 = Vec2::new(current.x, current.z);

    let diff = target_v2 - current_v2;
    let mag = f32::sqrt(diff.x.powi(2) + diff.y.powi(2));
    if mag <= step || mag == 0_f32 {

        return target;
    }

    let new = current_v2 + f32::sign
}
