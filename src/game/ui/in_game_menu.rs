use bevy::prelude::*;

use crate::{
    states::in_game::{InGameState, PauseState},
    core::setting::key_map,
    core::ui::style::TEXT_COLOR,
};

#[derive(Component)]
pub struct OnInGameMenuScreen;

pub fn toggle_in_game_menu(
    mut commands: Commands,
    query: Query<Entity, With<OnInGameMenuScreen>>,
    key_maps: Res<key_map::KeyMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    //InGameState
    in_game_state: Res<State<InGameState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    //PauseState
    pause_menu: Res<State<PauseState>>,
    mut next_pause_menu_state: ResMut<NextState<PauseState>>,
) {
    if keyboard_input.just_pressed(key_maps.toggle_in_game_menu) {
        // Toggle the in-game menu visibility
        match (in_game_state.get(), pause_menu.get()) {
            // If in-game Menu is open, close it
            (InGameState::Paused, PauseState::InGameMenu) => {
                if !query.is_empty() {
                    for entity in query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                // change the game state to playing and pause state to disabled
                next_pause_menu_state.set(PauseState::Disabled);
                next_in_game_state.set(InGameState::Playing);
            }
            // Only if the game is playing, we can open the in-game menu
            (InGameState::Playing, PauseState::Disabled) => {
                //spawn the in-game menu UI
                commands.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0., 0.1, 0.1, 0.5)),
                    Text("In Game Menu".to_string()),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    ZIndex(100),
                    OnInGameMenuScreen,
                ));
                // change the game state to paused and the pause state to in-game menu
                next_in_game_state.set(InGameState::Paused);
                next_pause_menu_state.set(PauseState::InGameMenu);
            }
            _ => {
                println!(">! Error: InGameState and PauseState do not match");
            }
        }
    }
}
