use bevy::prelude::*;

use crate::{core::resource::ActiveDatas, game::world::{map::components::PlayerMarker, NPCs::components::NPCMarker}};

pub fn detect_nearby_npc(
    player_query: Query<(&Transform, &PlayerMarker)>,
    npc_query: Query<(&Transform, &NPCMarker)>,
    mut active_datas: ResMut<ActiveDatas>,
) {
    if let Ok((player_transform, _)) = player_query.single() {
        let player_position = player_transform.translation;
        let mut npc_distances: Vec<(u32, f32)> = Vec::new();

        for (npc_transform, npc) in npc_query.iter() {
            let npc_position = npc_transform.translation;
            let distance = player_position.distance(npc_position);

            // 1.例えば、100.0の距離以内にいるNPCを検出する
            if distance <= 100.0 {
                npc_distances.push((npc.id, distance));
            }
        }
        // 2.最も近いNPCを見つける
        match closest_npc(&npc_distances){
            Some(npc_id) => {
                println!("Closest NPC is {}", npc_id);
                if active_datas.closest_npc != npc_id {
                    active_datas.closest_npc = npc_id;
                }
            },
            None => {
                println!("No nearby NPC found");
                active_datas.closest_npc = 0;
            },
        }
    }
}

fn closest_npc(npc_distances: &Vec<(u32, f32)>) -> Option<u32> {
    if npc_distances.is_empty() {
        return None;
    }

    let mut closest_npc = npc_distances[0];

    for &(npc_id, distance) in npc_distances.iter() {
        if distance < closest_npc.1 {
            closest_npc = (npc_id, distance);
        }
    }

    Some(closest_npc.0)
}