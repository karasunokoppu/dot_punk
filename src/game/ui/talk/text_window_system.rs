use bevy::prelude::*;

use crate::{core::resource::{ActiveDatas, Stages}, game::world::{player::interact_entity::controll::TalkToNPCEvent, NPCs::components::{NPCType, NPC}}};

//TODO [TalkToNPCEventが送信されたらテキストウィンドウを表示するシステムを実装する]
pub fn create_text_window(
    mut commands: Commands,
    mut talk_to_npc_event_reader: EventReader<TalkToNPCEvent>,
    r_active_datas: Res<ActiveDatas>,
    r_stages: Res<Stages>
){
    for event in talk_to_npc_event_reader.read(){
        for stage in r_stages.stage_list.iter(){
            if stage.id == r_active_datas.active_stage_id{
                for npc in stage.npcs.iter(){
                    if npc.id == event.npc_id{
                        match &npc.npc_type{
                            NPCType::Generic(talk_dialog) => {
                                //ここでテキストウィンドウを表示する処理を実装する
                                println!("talk to npc id: {}", npc.id);
                            },
                            NPCType::Merchant => {},
                            NPCType::QuestGiver => {}
                        }
                    }
                }
            }
        }
    }
}