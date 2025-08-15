use crate::{
    core::components::{Position, SpriteData},
    game::world::player::{components::Direction, states_components::EntityStates},
};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ActiveDatas {
    pub active_map_id: u32,
    pub active_map_name: String, //TODO [テレポート時に時に設定]
    pub teleport_map: u32,           // Next teleport map ID
    pub teleport_position: Position, // Next teleport node ID
}

#[derive(Resource)]
pub struct Player {
    pub name: String,
    pub sprite: SpriteData,
    pub position: Position,
    pub direction: Direction,
    pub states: EntityStates,
}
impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player Charactor01".to_string(),
            sprite: SpriteData {
                z_index: 10, //10..19
                image: "default_player_image.png".to_string(),
            },
            //TODO [更新プログラム作成]
            position: Position::default(),
            direction: Direction::default(),
            states: EntityStates::default(),
        }
    }
}
