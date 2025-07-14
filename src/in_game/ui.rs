pub mod in_game_menu;
pub mod pause_menu;

use bevy::prelude::*;

use crate::{in_game::InGameState, GameState};

pub fn in_game_ui_plugin(app: &mut App) {
    app.add_systems(Update, (
        pause_menu::toggle_pause_menu,
        in_game_menu::toggle_in_game_menu,
    ).run_if(in_state(GameState::InGame)));
}