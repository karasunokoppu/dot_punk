use bevy::prelude::*;

use crate::game::world::player::interact_entity::controll::TalkToNPCEvent;

//TODO [TalkToNPCEventが送信されたらテキストウィンドウを表示するシステムを実装する]
pub fn create_text_window(
    mut commands: Commands,
    mut talk_to_npc_event_reader: EventReader<TalkToNPCEvent>
){
    for event in talk_to_npc_event_reader.read(){
        println!("TalkToNPCEvent received! npc_id: {}", event.npm_id);
        //ここでテキストウィンドウを表示する処理を実装する
    }
}