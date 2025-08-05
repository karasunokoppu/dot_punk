use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{
    core::{resource::Player, setting::key_map::KeyMap},
    game::world::player::components::Direction,
};

pub fn change_sprite_direction(player_speed: Vec2, mut r_player: ResMut<Player>) {
    //TODO [プレイヤースプライトの更新を実装]
}
