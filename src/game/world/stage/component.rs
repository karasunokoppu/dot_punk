use bevy::prelude::*;

use crate::{
    core::components::Position,
    game::world::{npc::components::NPC, map::components::Map},
};

#[derive(Component)]
pub struct Stage {
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
                NPC::new(1).with_position(Position {x: -153.0, y: -109.0,}),
                NPC::new(2).with_position(Position { x: 20.0, y: -114.0 }),
            ],
        }
    }
}
