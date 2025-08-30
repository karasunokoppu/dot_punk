use bevy::{color::palettes::css::{LIGHT_GRAY, WHITE}, prelude::*};

use crate::{core::{resource::{ActiveDatas, Stages}, ui::style::{Talk_TextBox_NAME_COLOR, TEXT_COLOR}}, game::{ui::talk::{components::{TalkElementType, Talkers}, TalkTextBoxState}, world::{npc::components::NPCType, player::interact_entity::controll::TalkToNPCEvent}}};

#[derive(Component)]
pub struct TalkTextBoxMarker;

#[derive(Component)]
pub enum TalkTextBlocks{
    Name,
    Text,
}

//TODO [TalkToNPCEventが送信されたらテキストウィンドウを表示するシステムを実装する]
pub fn create_text_window(
    mut commands: Commands,
    mut talk_to_npc_event_reader: EventReader<TalkToNPCEvent>,
    r_active_datas: Res<ActiveDatas>,
    r_stages: Res<Stages>,
    mut next_talk_textbox_state: ResMut<NextState<TalkTextBoxState>>,
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
                                let first_talk_element = &talk_dialog.dialog.iter().find(|element|{element.local_talk_id == 0}).unwrap();
                                commands.spawn((
                                    Node {
                                        width: Val::Percent(80.0),
                                        height: Val::Percent(30.0),
                                        left: Val::Percent(10.0),
                                        top: Val::Percent(70.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    BackgroundColor(Color::BLACK.with_alpha(0.7)),
                                    BorderColor(LIGHT_GRAY.into()),
                                    TalkTextBoxMarker,
                                )).with_children(|parent|{
                                    match &first_talk_element.element_type{
                                        TalkElementType::Text(text) => {
                                            //テキストボックスの名前部分
                                            parent.spawn((
                                                Node{
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::FlexStart,
                                                    margin: UiRect {
                                                        left: Val::Px(3.0),
                                                        bottom: Val::Px(5.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                TalkTextBlocks::Name,
                                                Text(
                                                    match text.talker{
                                                        Talkers::Player => "Player".to_string(),
                                                        Talkers::NPC(npc_id) => {
                                                            match stage.npcs.iter().find(|n| n.id == npc_id){
                                                                Some(npc) => npc.name.clone(),
                                                                None => "Unknown".to_string(),
                                                            }
                                                        }
                                                    }
                                                ),
                                                TextFont {
                                                    font_size: 20.0,
                                                    ..default()
                                                },
                                                TextColor(Talk_TextBox_NAME_COLOR),
                                            ));
                                            //テキストボックスのテキスト部分
                                            parent.spawn((
                                                Node{
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::FlexStart,
                                                    margin: UiRect::left(Val::Px(3.0)),
                                                    ..default()
                                                },
                                                TalkTextBlocks::Text,
                                                Text(text.text.clone()),
                                                TextFont {
                                                    font_size: 20.0,
                                                    ..default()
                                                },
                                                TextColor(TEXT_COLOR),
                                            ));
                                        }
                                        TalkElementType::Choice(choices) => {
                                            //選択肢
                                        }
                                        TalkElementType::End => {}
                                    }
                                });
                                next_talk_textbox_state.set(TalkTextBoxState::Enabled);
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