use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{core::setting::game_setting::PLAYER_SPEED, game::world::{map::components::PlayerMarker, player::components::{Direction, Player}}};

pub fn move_player(
    mut player_vels: Query<&mut LinearVelocity, With<PlayerMarker>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut r_player: ResMut<Player>
) {
    let mut velocity = Vec2::ZERO;
    let mut new_direction = &r_player.direction;

    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += 1.0;
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

    if velocity == Vec2::new(1.0, 1.0){
        new_direction = &Direction::TopRight;
    }
    if velocity == Vec2::new(-1.0, 1.0){
        new_direction = &Direction::TopLeft;
    }
    if velocity == Vec2::new(1.0, -1.0){
        new_direction = &Direction::BottomRight;
    }
    if velocity == Vec2::new(-1.0, -1.0){
        new_direction = &Direction::BottomLeft;
    }
    if velocity == Vec2::Y{
        new_direction = &Direction::Top;
    }
    if velocity == Vec2::NEG_Y{
        new_direction = &Direction::Bottom;
    }
    if velocity == Vec2::X{
        new_direction = &Direction::Right;
    }
    if velocity == Vec2::NEG_X{
        new_direction = &Direction::Left;
    }
    //update Player.direction
    if r_player.direction != *new_direction{
        println!("{:?}", new_direction.clone());
        r_player.direction = new_direction.clone();
    }
}
