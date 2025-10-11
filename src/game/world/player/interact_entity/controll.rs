use bevy::prelude::*;

use crate::{
    core::{resource::ActiveDatas, setting::key_map::KeyMap},
    game::world::player::interact_entity::InteractEntities,
};

#[derive(Event)]
pub struct TalkToNPCEvent {
    pub npc_id: u32,
}

pub fn send_interact_event(
    key_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
    active_datas: Res<ActiveDatas>,
    mut talk_to_npc_event_writer: EventWriter<TalkToNPCEvent>,
) {
    if key_input.just_pressed(key_map.interact) {
        match &active_datas.closest_interact_entity_type {
            InteractEntities::Npc(npc) => {
                talk_to_npc_event_writer.write(TalkToNPCEvent { npc_id: npc.id });
            }
            InteractEntities::None => {}
        }
    }
}
