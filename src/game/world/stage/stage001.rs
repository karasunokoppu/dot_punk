use bevy::prelude::*;

use crate::{
    core::component::{Position, SpriteData},
    game::world::map::components::{Map, Maps, TeleportNode, WallColliderNode},
};

//How to register stage to the game
//1. Create a Map struct with the necessary data
//2. Push the Map in the Maps resource
//3. Register the stage in the map_plugin function

pub fn register_stage001(mut r_maps: ResMut<Maps>) {
    let map001: Map = Map {
        id: 1,
        name: "Map001".to_string(),
        sprites: vec![
            // スプライトデータの例
            SpriteData {
                z_index: 0, // 0..9
                image: "maps/stage001/map001.png".to_string(),
            },
            SpriteData {
                z_index: 20, // 20..29
                image: "maps/stage001/map001-fence.png".to_string(),
            },
        ],
        wall_colliders: vec![
            WallColliderNode {
                start_node: Position { x: 21.0, y: -93.0 },
                end_node: Position { x: 256.0, y: 24.0 },
            },
            WallColliderNode {
                start_node: Position { x: 21.0, y: -93.0 },
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
                id: 1,
                node_position: Position {
                    x: 300.0,
                    y: -200.0,
                },
                target_map: 0, // テレポート先のマップID
                teleport_position: Position {
                    x: -50.0,
                    y: -100.0,
                }, // テレポート先の座標
            },
        ],
    };
    // マップデータをロード
    r_maps.map_list.push(map001);
}
