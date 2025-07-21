use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{in_game::world::map::components::PlayerMarker, setting::game_setting::PLAYER_SPEED};

pub fn move_player(
    mut player_vels: Query<&mut LinearVelocity, With<PlayerMarker>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut velocity = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= 2.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += 2.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        velocity.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        velocity.y -= 1.0;
    }

    if let Ok(mut vel) = player_vels.single_mut() {
        vel.0 = velocity.normalize_or_zero() * PLAYER_SPEED;
    }
}
