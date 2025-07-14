use bevy::prelude::*;

use crate::{in_game::{InGameState, PauseState}, setting::key_map, utils::style::TEXT_COLOR};

#[derive(Component)]
pub struct OnPauseMenuScreen;

pub fn toggle_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<OnPauseMenuScreen>>,
    key_maps: Res<key_map::KeyMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // InGameState
    in_game_state: Res<State<InGameState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    // PauseState
    pause_menu: Res<State<PauseState>>,
    mut next_pause_menu_state: ResMut<NextState<PauseState>>,
) {
    if keyboard_input.just_pressed(key_maps.toggle_pause_menu) {
        match (in_game_state.get(), pause_menu.get()) {
            // If Pause Menu is open, close it
            (InGameState::Paused, PauseState::PauseMenu) => {
                if !query.is_empty() {
                    for entity in query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                // change the game state to playing and pause state to disabled
                next_pause_menu_state.set(PauseState::Disabled);
                next_in_game_state.set(InGameState::Playing);
            }
            // If the game is playing, we can open the pause menu
            (InGameState::Playing, PauseState::Disabled) => {
                // Otherwise, we can spawn the pause menu UI
                commands.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                    Text("Pause Menu".to_string()),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    OnPauseMenuScreen,
                ));
                // change the game state to paused and the pause state to pause menu
                next_in_game_state.set(InGameState::Paused);
                next_pause_menu_state.set(PauseState::PauseMenu);
            }
            _ => {println!(">! Error: InGameState and PauseState do not match");}
        }
    }
}