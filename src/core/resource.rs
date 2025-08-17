use crate::{
    core::components::{Position, SpriteData},
    game::world::{map::components::Map, player::{components::Direction, states_components::EntityStates}, NPCs::talk::TalkDialog},
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct ActiveDatas {
    pub active_map_id: u32,
    pub active_map_name: String, //TODO [テレポート時に時に設定]
    pub teleport_map: u32,           // Next teleport map ID
    pub teleport_position: Position, // Next teleport node ID
}
impl Default for ActiveDatas {
    fn default() -> Self {
        ActiveDatas {
            active_map_id: 0,
            active_map_name: "Default Map".to_string(),
            teleport_map: 0,
            teleport_position:Position {x: -50.0, y: -100.0}
        }
    }
}

#[derive(Resource)]
pub struct Maps {
    pub map_list: Vec<Map>,
}

#[derive(Resource)]
pub struct TalkDialogs {
    pub dialog_list: Vec<TalkDialog>,
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