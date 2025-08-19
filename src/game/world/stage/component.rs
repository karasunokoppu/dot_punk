use bevy::prelude::*;

use crate::{core::components::Position, game::world::{map::components::Map, NPCs::components::NPC}};

#[derive(Component)]
pub struct Stage{
    pub id: u32,
    pub name: String,
    pub map: Map,
    pub npcs: Vec<NPC>,
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            id: 0,
            name: "Default Stage".to_string(),
            map: Map::default(),
            npcs: vec![
                NPC {
                    id: 1,
                    name: "Citizen 01".to_string(),
                    position: Position { x: -153.0, y: -109.0 },
                    ..default()
                },
                NPC {
                    id: 2,
                    name: "Citizen 02".to_string(),
                    position: Position { x: 20.0, y: -114.0 },
                    ..default()
                },
            ]
        }
    }
}