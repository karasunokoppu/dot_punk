use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    MainCamera,
    core::{
        components::Position,
        resource::Player,
        setting::{
            game_setting::{PLAYER_RUN_SPEED, PLAYER_WARK_SPEED},
            key_map::KeyMap,
        },
    },
    game::world::{map::components::PlayerMarker, player::components::Direction},
    states::in_game::player_states::{ActionStates, MoveStates},
};

pub fn move_player(
    mut player_vels: Query<&mut LinearVelocity, With<PlayerMarker>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
    mut r_player: ResMut<Player>,
    mut action_states: ResMut<NextState<ActionStates>>,
    mut next_move_states: ResMut<NextState<MoveStates>>,
    move_states: Res<State<MoveStates>>,
) {
    let mut velocity = Vec2::ZERO;
    let mut new_direction = &r_player.direction;
    //**set player's speed**
    //get key input and set speed
    if keyboard_input.pressed(key_map.move_left) {
        velocity.x -= 2.0;
    }
    if keyboard_input.pressed(key_map.move_right) {
        velocity.x += 2.0;
    }
    if keyboard_input.pressed(key_map.move_up) {
        velocity.y += 1.0;
    }
    if keyboard_input.pressed(key_map.move_down) {
        velocity.y -= 1.0;
    }

    //set player speed
    if let Ok(mut vel) = player_vels.single_mut() {
        if velocity.x != 0.0 || velocity.y != 0.0 {
            action_states.set(ActionStates::Move);
        } else {
            next_move_states.set(MoveStates::Wark);
            action_states.set(ActionStates::Stand);
        }

        let dash_speed = match *move_states.get() {
            MoveStates::Wark => PLAYER_WARK_SPEED,
            MoveStates::Run => PLAYER_RUN_SPEED,
        };
        vel.0 = velocity.normalize_or_zero() * dash_speed;
    }

    //**update player direction data**
    if velocity == Vec2::new(1.0, 1.0) {
        new_direction = &Direction::TopRight;
    }
    if velocity == Vec2::new(-1.0, 1.0) {
        new_direction = &Direction::TopLeft;
    }
    if velocity == Vec2::new(1.0, -1.0) {
        new_direction = &Direction::BottomRight;
    }
    if velocity == Vec2::new(-1.0, -1.0) {
        new_direction = &Direction::BottomLeft;
    }
    if velocity == Vec2::Y {
        new_direction = &Direction::Top;
    }
    if velocity == Vec2::NEG_Y {
        new_direction = &Direction::Bottom;
    }
    if velocity == Vec2::X {
        new_direction = &Direction::Right;
    }
    if velocity == Vec2::NEG_X {
        new_direction = &Direction::Left;
    }
    //update Player.direction
    if r_player.direction != *new_direction {
        r_player.direction = new_direction.clone();
    }
}

pub fn update_player_pos_resource(
    player_vels: Query<(&Transform, &LinearVelocity), With<PlayerMarker>>,
    mut r_player: ResMut<Player>,
) {
    for (transform, vel) in player_vels.iter() {
        if vel.0 != Vec2::ZERO {
            r_player.position = Position {
                x: transform.translation.x,
                y: transform.translation.y,
            };
        }
    }
}

pub fn dash_mode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
    mut move_state: ResMut<NextState<MoveStates>>,
) {
    if keyboard_input.pressed(key_map.run) {
        move_state.set(MoveStates::Run);
    } else {
        move_state.set(MoveStates::Wark);
    }
}

pub fn update_camera_pos(
    player_pos: Query<&Transform, (With<PlayerMarker>, Without<MainCamera>)>,
    mut camera: Query<&mut Transform, (With<MainCamera>, Without<PlayerMarker>)>,
) {
    if let Ok(player_pos) = player_pos.single()
        && let Ok(mut camera_transform) = camera.single_mut()
    {
        camera_transform.translation.x = player_pos.translation.x;
        camera_transform.translation.y = player_pos.translation.y;
    }
}
