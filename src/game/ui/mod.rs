pub mod in_game_menu;
pub mod pause_menu;
pub mod setting_ui;
pub mod talk;

use bevy::prelude::*;

use crate::{
    GameState,
    core::{systems::despawn_screen, ui},
    game::ui::{
        pause_menu::PauseButtonAction,
        setting_ui::{OnPauseSettingsMenuScreen, pause_setting_menu_setup, setting_menu_action},
    },
    states::in_game::PauseState,
};

pub fn in_game_ui_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            ui::button::button_system,
            pause_menu::pause_menu_action,
            pause_menu::toggle_pause_menu,
            in_game_menu::toggle_in_game_menu,
        )
            .run_if(in_state(GameState::InGame)),
    )
    .add_systems(
        OnExit(PauseState::InGameMenu),
        despawn_screen::<in_game_menu::OnInGameMenuScreen>,
    )
    .add_systems(
        OnExit(PauseState::PauseMenu),
        despawn_screen::<pause_menu::OnPauseMenuScreen>,
    )
    .add_systems(
        OnEnter(PauseButtonAction::Settings),
        pause_setting_menu_setup,
    )
    .add_systems(
        Update,
        (setting_menu_action).run_if(in_state(PauseButtonAction::Settings)),
    )
    .add_systems(
        OnExit(PauseButtonAction::Settings),
        despawn_screen::<OnPauseSettingsMenuScreen>,
    );
}
