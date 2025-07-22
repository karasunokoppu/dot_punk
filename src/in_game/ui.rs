pub mod in_game_menu;
pub mod pause_menu;

use bevy::prelude::*;

use crate::{
    GameState, despawn_screen,
    in_game::PauseState,
    utils,
};

pub fn in_game_ui_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            utils::button::button_system,
            pause_menu::toggle_pause_menu,
            pause_menu::pause_menu_action,
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
    );
}
