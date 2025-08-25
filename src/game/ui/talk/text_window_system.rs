use bevy::prelude::*;

use crate::game::world::NPCs::components::NPC;

//TODO [内容変更]
#[derive(Event)]
pub struct TalkEvent {
    pub npc_id: u32,
    radius: f32,
}

// TalkEventを発生させる
pub fn handle_talk_event(
    key_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
){
    //TODO [内容をNPCがクリックさせたときに発生させるようにする]
    if key_input.just_pressed(KeyCode::KeyT){
        println!("Talk toriggered");
        commands.trigger(TalkEvent { npc_id: 1, radius: 100.0 });
    }
}

pub fn generate_text_window(
    trigger: Trigger<TalkEvent>,
    q_npc: Query<&NPC>
){
    if let Ok(target_npc) = q_npc.get(trigger.target()){
        println!("TalkEvent target NPC found: id={}, name={}", target_npc.id, target_npc.name);
    } else {
        println!("TalkEvent occurred error");
        return;
    }
    //TODO [テキストウィンドウ生成処理を実装]
    // println!("TalkEvent triggered: npc_id={}, radius={}", trigger.npc_id, trigger.radius);
}