pub mod components;
pub mod text_window_system;

use bevy::prelude::*;

use crate::{core::systems::despawn_screen, game::ui::talk::text_window_system::{read_talk_text, TalkTextBoxMarker}, GameState};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TalkTextBoxState{
    #[default]
    Disabled,
    Enabled,
}

pub fn talk_ui_plugin(app: &mut App) {
    app
    .init_state::<TalkTextBoxState>()
    .add_systems(OnEnter(TalkTextBoxState::Disabled), despawn_screen::<TalkTextBoxMarker>)
    .add_systems(Update, text_window_system::create_text_window.run_if(in_state(GameState::InGame).and(in_state(TalkTextBoxState::Disabled))))
    .add_systems(Update, read_talk_text.run_if(in_state(TalkTextBoxState::Enabled)));
}