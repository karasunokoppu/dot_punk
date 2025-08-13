use bevy::prelude::*;

use crate::{core::{systems::despawn_screen, ui::style::TEXT_COLOR}, GameState};

pub fn debug_plungin(app: &mut App) {
    app.add_systems(OnEnter(DebugModeState::On), spawn_debug_information)
        .add_systems(OnExit(DebugModeState::On),despawn_screen::<DebugModeMarker>)
        .add_systems(Update, toggle_debug_mode.run_if(in_state(GameState::InGame)));
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DebugModeState {
    #[default]
    Off,
    On,
}

#[derive(Component)]
pub struct DebugModeMarker;

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
            Text("Debug Mode".to_string()),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
        //TODO [URLのサンプルを参考にデバッグ情報の表示を実装。https://bevy.org/examples/ui-user-interface/text/]
        parent.spawn((
            Text("Debug information".to_string()),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        ));
    });
}

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
