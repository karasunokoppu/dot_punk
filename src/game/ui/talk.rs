pub mod components;
pub mod text_window_system;

use bevy::prelude::*;

use crate::GameState;

pub fn talk_ui_plugin(app: &mut App) {
    app.add_systems(Update, text_window_system::create_text_window.run_if(in_state(GameState::InGame)));
}