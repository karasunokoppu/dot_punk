use bevy::{color::palettes::css::LIGHT_GRAY, prelude::*};

use crate::{
    core::{
        resource::{ActiveDatas, Stages},
        setting::key_map::KeyMap,
        ui::style::{TALK_TEXTBOX_NAME_COLOR, TEXT_COLOR},
    },
    game::{
        ui::talk::{
            TalkTextBoxChoiceIndex, TalkTextBoxState, TalkTextBoxType,
            components::{
                TalkChoiceElement, TalkDialog, TalkElementType, TalkTextElement, Talkers,
            },
        },
        world::{
            npc::components::NPCType, player::interact_entity::controll::TalkToNPCEvent,
            stage::component::Stage,
        },
    },
};

#[derive(Component)]
pub struct TalkTextBoxMarker;

#[derive(Component)]
pub struct TalkTextBoxChoiceMarker {
    choice_id: u32,
    next_choice_id: u32,
}

#[derive(Component)]
pub enum TalkTextBlocks {
    Name,
    Text,
}

pub fn create_text_window(
    mut commands: Commands,
    mut talk_to_npc_event_reader: EventReader<TalkToNPCEvent>,
    mut r_active_datas: ResMut<ActiveDatas>,
    r_stages: Res<Stages>,
    mut next_talk_textbox_state: ResMut<NextState<TalkTextBoxState>>,
    mut next_talk_textbox_type: ResMut<NextState<TalkTextBoxType>>,
    mut r_talk_textbox_choice_index: ResMut<TalkTextBoxChoiceIndex>,
) {
    for event in talk_to_npc_event_reader.read() {
        for stage in r_stages.stage_list.iter() {
            if stage.id == r_active_datas.active_stage_id {
                for npc in stage.npcs.iter() {
                    if npc.id == event.npc_id {
                        match &npc.npc_type {
                            NPCType::Generic(talk_dialog) => {
                                println!("> talk to npc id: {}", npc.id);
                                //会話データを持っているNPCのIDを保持
                                r_active_datas.talking_npc = Some(npc.id);
                                //最初の会話データを取得
                                let first_talk_element = &talk_dialog
                                    .dialog
                                    .iter()
                                    .find(|element| element.local_talk_id == 0)
                                    .unwrap();
                                next_talk_textbox_state.set(TalkTextBoxState::Enabled);
                                //テキストボックス表示
                                match &first_talk_element.element_type {
                                    //TODO [memo: first_talk_element.element_type以外同一]
                                    TalkElementType::Text(text) => {
                                        spawn_talk_textbox_text_ui(
                                            &mut commands,
                                            text,
                                            stage,
                                            &mut r_active_datas,
                                            talk_dialog,
                                            &mut next_talk_textbox_type,
                                            &mut r_talk_textbox_choice_index,
                                        );
                                    }
                                    TalkElementType::Choice(choices) => {
                                        spawn_talk_textbox_choice_ui(
                                            &mut commands,
                                            &mut r_active_datas,
                                            choices,
                                            &mut r_talk_textbox_choice_index,
                                        );
                                    }
                                    TalkElementType::End => {
                                        next_talk_textbox_state.set(TalkTextBoxState::Disabled);
                                        r_active_datas.talk_index = None;
                                        r_active_datas.talking_npc = None;
                                        println!("> talk finished");
                                    }
                                }
                            }
                            NPCType::Merchant => {}
                            NPCType::QuestGiver => {}
                        }
                    }
                }
            }
        }
    }
}

pub fn read_talk_text(
    mut commands: Commands,
    text_box: Query<Entity, With<TalkTextBoxMarker>>,
    mut r_active_datas: ResMut<ActiveDatas>,
    r_stages: Res<Stages>,
    key_map: Res<KeyMap>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut next_talk_textbox_state: ResMut<NextState<TalkTextBoxState>>,
    mut next_talk_textbox_type: ResMut<NextState<TalkTextBoxType>>,
    mut r_talk_textbox_choice_index: ResMut<TalkTextBoxChoiceIndex>,
) {
    if key_input.just_pressed(key_map.advance_text) {
        // 1. テキストボックスをリセット
        if let Ok(text_box_entity) = text_box.single() {
            commands.entity(text_box_entity).despawn();
        }
        // 2. テキストボックスをリライト
        // 2-1. 会話対象のNPCを特定する
        for stage in r_stages.stage_list.iter() {
            if stage.id == r_active_datas.active_stage_id {
                for npc in stage.npcs.iter() {
                    match r_active_datas.talking_npc {
                        Some(npc_id) => {
                            if npc.id == npc_id {
                                // 2-2. テキストボックスを再構成して表示
                                match &npc.npc_type {
                                    NPCType::Generic(talk_dialog) => {
                                        //ここでテキストウィンドウを表示する処理を実装する
                                        println!(
                                            "> talk index: {}",
                                            r_active_datas.talk_index.unwrap()
                                        );
                                        //会話データを持っているNPCのIDを保持
                                        r_active_datas.talking_npc = Some(npc.id);
                                        //最初の会話データを取得
                                        let next_talk_index =
                                            r_active_datas.talk_index.unwrap_or_default();
                                        let next_talk_element = &talk_dialog
                                            .dialog
                                            .iter()
                                            .find(|element| {
                                                element.local_talk_id == next_talk_index
                                            })
                                            .unwrap();
                                        match &next_talk_element.element_type {
                                            TalkElementType::Text(text) => {
                                                spawn_talk_textbox_text_ui(
                                                    &mut commands,
                                                    text,
                                                    stage,
                                                    &mut r_active_datas,
                                                    talk_dialog,
                                                    &mut next_talk_textbox_type,
                                                    &mut r_talk_textbox_choice_index,
                                                );
                                            }
                                            TalkElementType::Choice(choices) => {
                                                spawn_talk_textbox_choice_ui(
                                                    &mut commands,
                                                    &mut r_active_datas,
                                                    choices,
                                                    &mut r_talk_textbox_choice_index,
                                                );
                                            }
                                            TalkElementType::End => {
                                                next_talk_textbox_state
                                                    .set(TalkTextBoxState::Disabled);
                                                r_active_datas.talk_index = None;
                                                r_active_datas.talking_npc = None;
                                                println!("> talk finished");
                                            }
                                        }
                                    }
                                    NPCType::Merchant => {}
                                    NPCType::QuestGiver => {}
                                }
                            }
                        }
                        None => {
                            //TODO [エラー処理を実装する]
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_talk_textbox_text_ui(
    commands: &mut Commands,
    text: &TalkTextElement,
    stage: &Stage,
    r_active_datas: &mut ResMut<ActiveDatas>,
    talk_dialog: &TalkDialog,
    next_talk_textbox_type: &mut ResMut<NextState<TalkTextBoxType>>,
    r_talk_textbox_choice_index: &mut ResMut<TalkTextBoxChoiceIndex>,
) {
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            //テキストボックスの名前部分
            parent.spawn((
                Node {
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
                Text(match text.talker {
                    Talkers::Player => "Player".to_string(),
                    Talkers::NPC(npc_id) => match stage.npcs.iter().find(|n| n.id == npc_id) {
                        Some(npc) => npc.name.clone(),
                        None => "Unknown".to_string(),
                    },
                }),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TALK_TEXTBOX_NAME_COLOR),
            ));
            //テキストボックスのテキスト部分
            parent.spawn((
                Node {
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
        });
    r_active_datas.talk_index = Some(text.next_talk_element_id);
    let next_talk_element = &talk_dialog
        .dialog
        .iter()
        .find(|element| element.local_talk_id == text.next_talk_element_id)
        .unwrap();
    match &next_talk_element.element_type {
        TalkElementType::Text(_) => next_talk_textbox_type.set(TalkTextBoxType::Text),
        TalkElementType::Choice(choice_elements) => {
            let next_first_choice = choice_elements.first().unwrap();
            next_talk_textbox_type.set(TalkTextBoxType::Choice(
                next_first_choice.next_talk_element_id,
            ));
            r_talk_textbox_choice_index.0 = 1;
        }
        TalkElementType::End => next_talk_textbox_type.set(TalkTextBoxType::Disabled),
    }
}

pub fn spawn_talk_textbox_choice_ui(
    commands: &mut Commands,
    r_active_datas: &mut ResMut<ActiveDatas>,
    choices: &Vec<TalkChoiceElement>,
    r_talk_textbox_choice_index: &mut ResMut<TalkTextBoxChoiceIndex>,
) {
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            for choice in choices {
                parent.spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        margin: UiRect::left(Val::Px(3.0)),
                        ..default()
                    },
                    TalkTextBoxChoiceMarker {
                        choice_id: choice.choice_id,
                        next_choice_id: choice.next_talk_element_id,
                    },
                    Text(choice.text.clone()),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(if choice.choice_id == r_talk_textbox_choice_index.0 {
                        println!("index {}is selected", choice.choice_id);
                        TALK_TEXTBOX_NAME_COLOR
                    } else {
                        TEXT_COLOR
                    }),
                ));
            }
        });
    r_active_datas.talk_index = Some(choices.first().unwrap().next_talk_element_id);
    r_talk_textbox_choice_index.0 = 1;
}

pub fn flip_choice_color(
    mut q_choice_text_color: Query<(&mut TextColor, &TalkTextBoxChoiceMarker)>,
    key_map: Res<KeyMap>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut r_talk_textbox_choice_index: ResMut<TalkTextBoxChoiceIndex>,
    mut r_active_datas: ResMut<ActiveDatas>,
) {
    if key_input.just_pressed(key_map.interact) {
        let mut found = false;
        for (mut text_color, marker) in q_choice_text_color.iter_mut() {
            if marker.choice_id == r_talk_textbox_choice_index.0 + 1 {
                *text_color = TextColor(TALK_TEXTBOX_NAME_COLOR);
                r_talk_textbox_choice_index.0 = marker.choice_id;
                r_active_datas.talk_index = Some(marker.next_choice_id);
                found = true;
            } else {
                *text_color = TextColor(TEXT_COLOR);
            }
        }

        if !found {
            // fallback: 最初の選択肢に戻す
            for (mut text_color, marker) in q_choice_text_color.iter_mut() {
                if marker.choice_id == 1 {
                    *text_color = TextColor(TALK_TEXTBOX_NAME_COLOR);
                    r_talk_textbox_choice_index.0 = marker.choice_id;
                    r_active_datas.talk_index = Some(marker.next_choice_id);
                } else {
                    *text_color = TextColor(TEXT_COLOR);
                }
            }
        }
    }
}
