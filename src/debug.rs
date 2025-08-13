use bevy::prelude::*;

use crate::{core::{resource::Player, systems::despawn_screen, ui::style::TEXT_COLOR}, GameState};

pub fn debug_plungin(app: &mut App) {
    app.add_systems(OnEnter(DebugModeState::On), spawn_debug_information)
        .add_systems(OnExit(DebugModeState::On),despawn_screen::<DebugModeMarker>)
        .add_systems(Update, toggle_debug_mode.run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            update_debug_info_player_position,
        ).run_if(in_state(DebugModeState::On)));
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DebugModeState {
    #[default]
    Off,
    On,
}

#[derive(Component)]
pub enum DebugInfoMarker{
    PlayerPosition,
    PlayerDirection,
    PlayerStatesHP,
    PlayerStatesMP,
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

pub fn spawn_debug_information(mut commands: Commands) {
    commands.spawn((
        DebugModeMarker,
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Baseline,
            ..default()
        },
        BackgroundColor(Color::BLACK.with_alpha(0.4)),
        GlobalZIndex(200),
    )).with_children(|parent| {
        parent.spawn((
            Text("Player".to_string()),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
        //TODO [URLのサンプルを参考にデバッグ情報の表示を実装。https://bevy.org/examples/ui-user-interface/text/]
        parent.spawn((
            Text("player position: ".to_string()),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        )).with_children(|parent|{
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
    });
}

pub fn update_debug_info_player_position(
    player: Res<Player>,
    mut query: Query<&mut TextSpan, With<DebugInfoMarker>>,
) {
    if let Ok(mut text_span) = query.single_mut() {
        **text_span = format!("x: {}, y: {}", player.position.x, player.position.y);
    }
}