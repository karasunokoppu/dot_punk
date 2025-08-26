use bevy::prelude::*;

use crate::{
    core::{
        resource::{ActiveDatas, Player},
        systems::despawn_screen,
        ui::style::TEXT_COLOR,
    }, game::world::player::activate_entity::ActivateEntities, GameState
};

pub fn debug_plungin(app: &mut App) {
    app.add_systems(OnEnter(DebugModeState::On), spawn_debug_information)
        .add_systems(
            OnExit(DebugModeState::On),
            despawn_screen::<DebugModeMarker>,
        )
        .add_systems(
            Update,
            toggle_debug_mode.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (update_debug_info,).run_if(in_state(DebugModeState::On)),
        );
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DebugModeState {
    #[default]
    Off,
    On,
}

#[derive(Component, PartialEq)]
pub enum DebugInfoMarker {
    PlayerPosition,
    PlayerDirection,
    ClosestActivateEntity,
    PlayerStatesHP,
    PlayerStatesMP,
    MapName,
}

#[derive(Component)]
pub struct DebugModeMarker;

pub fn toggle_debug_mode(
    mut next_debug_mode_state: ResMut<NextState<DebugModeState>>,
    debug_mode_state: Res<State<DebugModeState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        if *debug_mode_state.get() == DebugModeState::On {
            next_debug_mode_state.set(DebugModeState::Off);
        } else {
            next_debug_mode_state.set(DebugModeState::On);
        }
    }
}

pub fn spawn_debug_information(mut commands: Commands, r_player: Res<Player>) {
    commands
        .spawn((
            DebugModeMarker,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Baseline,
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.4)),
            GlobalZIndex(200),
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|parent| {
                    // Player informations
                    parent.spawn((
                        Text("Player".to_string()),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                    parent.spawn((
                        Text(format!("player name: {}", r_player.name)),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                    ));
                    // Player position
                    parent
                        .spawn((
                            Text("player position: ".to_string()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::default(),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                                DebugInfoMarker::PlayerPosition,
                            ));
                        });
                    // Player direction
                    parent
                        .spawn((
                            Text("player direction: ".to_string()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::default(),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                                DebugInfoMarker::PlayerDirection,
                            ));
                        });
                    // closest NPC
                    parent
                        .spawn((
                            Text("closest activate entity: ".to_string()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::default(),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                                DebugInfoMarker::ClosestActivateEntity,
                            ));
                        });
                    //TODO Player states HP, MP, etcのデバッグ情報を追加
                });
            // Map informations
            parent.spawn((
                Text("Maps".to_string()),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|parent| {
                    // Map name
                    parent
                        .spawn((
                            Text("Map: ".to_string()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::default(),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                                DebugInfoMarker::MapName,
                            ));
                        });
                });
        });
}

pub fn update_debug_info(
    active_datas: Res<ActiveDatas>,
    r_player: Res<Player>,
    mut query: Query<(&mut TextSpan, &DebugInfoMarker)>,
) {
    for (mut text_span, debug_info) in query.iter_mut() {
        match *debug_info {
            DebugInfoMarker::PlayerPosition => {
                **text_span = format!("x: {}, y: {}", r_player.position.x, r_player.position.y)
            }
            DebugInfoMarker::PlayerDirection => {
                **text_span = format!("direction: {:?}", r_player.direction)
            }
            DebugInfoMarker::MapName => {
                **text_span = format!("map name: {}", active_datas.active_stage_name)
            }
            DebugInfoMarker::ClosestActivateEntity => {
                **text_span = match &active_datas.closest_activate_entity_type{
                    ActivateEntities::NPC(npc) => format!("NPC : {}", npc.id),
                    ActivateEntities::None => "None".to_string(),
                }
            }
            _ => {}
        }
    }
}
