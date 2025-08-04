use avian2d::prelude::CollidingEntities;
use bevy::prelude::*;

use crate::{
    core::resource::ActiveDatas,
    game::world::map::components::{TeleportNode, TeleportNodeMarker},
    states::in_game::InGameState,
};

pub fn detect_teleport_node_colliding(
    query: Query<(Entity, &CollidingEntities)>,
    teleport_node: Query<(Entity, &TeleportNode), With<TeleportNodeMarker>>,
    mut r_active_datas: ResMut<ActiveDatas>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    for (_entity, colliding_entities) in &query {
        if colliding_entities.is_empty() {
            continue;
        }
        //detect colliding teleport nodes
        for (teleport_entity, teleport_node) in teleport_node.iter() {
            if colliding_entities.contains(&teleport_entity) {
                // Update next teleport map and position
                r_active_datas.teleport_map = teleport_node.target_map;
                r_active_datas.teleport_position = teleport_node.teleport_position;
                // Set the next state to loading to handle the teleport
                in_game_state.set(InGameState::Loading);
                println!("Player teleport to: {:?}", teleport_node.target_map);
            }
        }
    }
}
