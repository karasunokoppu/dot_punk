use bevy::prelude::*;

use crate::{
    core::{resource::ActiveDatas, setting::game_setting::PLAYER_TALKABLE_LENGTH},
    game::world::{
        map::components::PlayerMarker,
        player::interact_entity::{InteractEntities, NPCMarker},
    },
};

pub fn detect_nearby_activate_entity(
    player_query: Query<(&Transform, &PlayerMarker)>,
    activate_entity_query: Query<(&Transform, &InteractEntities)>,
    mut active_datas: ResMut<ActiveDatas>,
) {
    if let Ok((player_transform, _)) = player_query.single() {
        let player_position = player_transform.translation;
        let mut activate_entity_distances: Vec<(InteractEntities, f32)> = Vec::new();

        // 1.すべてのActivate Entityとの距離を計算する
        for (activate_entity_transform, activate_entity) in activate_entity_query.iter() {
            let activate_entity_position = activate_entity_transform.translation;
            let distance = player_position.distance(activate_entity_position);

            //1-1.各Activate Entityのタイプごとに適切な距離をチェックする
            match activate_entity {
                InteractEntities::Npc(npc) => {
                    // 100.0の距離以内にいるNPCを検出する
                    if distance <= PLAYER_TALKABLE_LENGTH {
                        activate_entity_distances
                            .push((InteractEntities::Npc(NPCMarker { id: npc.id }), distance));
                    }
                }
                InteractEntities::None => {}
            }
        }
        // 2.最も近いActivate Entityを見つける
        match closest_activate_entity(activate_entity_distances) {
            Some(active_entity) => {
                if active_datas.closest_interact_entity_type != active_entity {
                    active_datas.closest_interact_entity_type = active_entity;
                }
            }
            None => {
                active_datas.closest_interact_entity_type = InteractEntities::None;
            }
        }
    }
}

fn closest_activate_entity(
    npc_distances: Vec<(InteractEntities, f32)>,
) -> Option<InteractEntities> {
    if npc_distances.is_empty() {
        return None;
    }

    let mut closest_activate_entity = npc_distances[0].clone();

    for (activate_entity, distance) in npc_distances.iter() {
        if *distance < closest_activate_entity.1 {
            closest_activate_entity = (activate_entity.clone(), *distance);
        }
    }

    Some(closest_activate_entity.0)
}
