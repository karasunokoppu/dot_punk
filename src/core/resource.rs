use crate::{
    core::components::{Position, SpriteData},
    game::world::{
        map::components::Map,
        player::{components::Direction, states_components::EntityStates},
        stage::component::Stage,
    },
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct ActiveDatas {
    pub active_stage_id: u32,
    pub active_stage_name: String,
    pub teleport_stage: u32,         // Next teleport map ID
    pub teleport_position: Position, // Next teleport node ID
    pub closest_npc: u32,        // Closest NPC ID
}
impl Default for ActiveDatas {
    fn default() -> Self {
        ActiveDatas {
            active_stage_id: 0,
            active_stage_name: "Default Map".to_string(),
            teleport_stage: 0,
            teleport_position: Position {
                x: -50.0,
                y: -100.0,
            },
            closest_npc: 0,
        }
    }
}

#[derive(Resource)]
pub struct Stages {
    pub stage_list: Vec<Stage>,
}
impl Default for Stages {
    fn default() -> Self {
        Stages {
            stage_list: vec![Stage::default()],
        }
    }
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
            position: Position::default(),
            direction: Direction::default(),
            //TODO [更新プログラム作成]
            states: EntityStates::default(),
        }
    }
}
