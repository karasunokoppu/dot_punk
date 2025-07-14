use bevy::prelude::*;

use crate::in_game::world::{Position, SpriteData};

#[derive(Resource)]
pub struct Maps {
    pub map_list: Vec<Map>,
}

#[derive(Component)]
pub struct Map {
    pub id: u32,
    pub name: String,
    pub sprites: SpriteData,
    pub wall_colliders: Vec<WallColliderNode>,
    pub teleport_nodes: Vec<TeleportNode>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            id: 0,
            name: String::from("Default Map"),
            sprites: SpriteData {
                z_index: 0,
                image: String::from("maps/map000.png"),
            },
            wall_colliders: vec![
                WallColliderNode {
                    start_node: Position { x: 224.0, y: -194.0 },
                    end_node: Position { x: -255.0, y: 45.0 },
                },
                WallColliderNode {
                    start_node: Position { x: -255.0, y: 45.0 },
                    end_node: Position { x: 256.0, y: -177.0 },
                },
                WallColliderNode {
                    start_node: Position { x: -256.0, y: -81.0 },
                    end_node: Position { x: 93.0, y: -256.0 },
                },
            ],
            teleport_nodes: Vec::new(),
        }
    }
}

#[derive(Component, Default)]
pub struct WallColliderNode {
    pub start_node: Position,
    pub end_node: Position,
}

#[derive(Component, Default)]
pub struct TeleportNode {
    pub id : u8,
    pub target_map: u32,
    pub target_node: Position,
}