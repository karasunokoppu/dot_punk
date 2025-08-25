use bevy::prelude::*;

use crate::{
    core::components::{Position, SpriteData},
    game::world::NPCs::components::NPC,
};

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct TeleportNodeMarker;

#[derive(Component)]
pub struct Map {
    pub sprites: Vec<SpriteData>,
    pub wall_colliders: Vec<WallColliderNode>,
    pub teleport_nodes: Vec<TeleportNode>,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            sprites: vec![
                SpriteData {
                    z_index: 0, //0..9
                    image: String::from("maps/stage000/map000.png"),
                },
                SpriteData {
                    z_index: 20, //20..29
                    image: String::from("maps/stage000/map000-fence.png"),
                },
            ],
            wall_colliders: vec![
                WallColliderNode {
                    start_node: Position {
                        x: 224.0,
                        y: -194.0,
                    },
                    end_node: Position {
                        x: 256.0,
                        y: -177.0,
                    },
                },
                WallColliderNode {
                    start_node: Position {
                        x: 224.0,
                        y: -194.0,
                    },
                    end_node: Position { x: -255.0, y: 45.0 },
                },
                WallColliderNode {
                    start_node: Position {
                        x: -256.0,
                        y: -81.0,
                    },
                    end_node: Position { x: 93.0, y: -256.0 },
                },
            ],
            teleport_nodes: vec![
                // テレポートノードの例
                TeleportNode {
                    id: 0, // 全マップ共通のユニークなID
                    node_position: Position {
                        x: 300.0,
                        y: -200.0,
                    },
                    target_stage: 1, // テレポート先のマップID
                    teleport_position: Position {
                        x: -50.0,
                        y: -100.0,
                    }, // テレポート先の座標
                },
            ],
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
    pub id: u8,
    pub node_position: Position,
    pub target_stage: u32,
    pub teleport_position: Position,
}
