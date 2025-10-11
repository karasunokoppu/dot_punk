pub mod components;
pub mod text_window_system;

use bevy::prelude::*;

use crate::{
    GameState,
    core::systems::despawn_screen,
    game::ui::talk::text_window_system::{TalkTextBoxMarker, flip_choice_color, read_talk_text},
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TalkTextBoxState {
    #[default]
    Disabled,
    Enabled,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum TalkTextBoxType {
    #[default]
    Disabled,
    Text,
    Choice(u32),
}

#[derive(Resource)]
pub struct TalkTextBoxChoiceIndex(u32);
impl Default for TalkTextBoxChoiceIndex {
    fn default() -> Self {
        TalkTextBoxChoiceIndex(1)
    }
}

pub fn talk_ui_plugin(app: &mut App) {
    app.init_state::<TalkTextBoxState>()
        .init_state::<TalkTextBoxType>()
        .init_resource::<TalkTextBoxChoiceIndex>()
        .add_systems(
            OnEnter(TalkTextBoxState::Disabled),
            despawn_screen::<TalkTextBoxMarker>,
        )
        .add_systems(
            Update,
            text_window_system::create_text_window
                .run_if(in_state(GameState::InGame).and(in_state(TalkTextBoxState::Disabled))),
        )
        .add_systems(
            Update,
            read_talk_text.run_if(in_state(TalkTextBoxState::Enabled)),
        )
        .add_systems(
            Update,
            flip_choice_color.run_if(not(
                in_state(TalkTextBoxType::Disabled).and(in_state(TalkTextBoxType::Text))
            )),
        );
}
