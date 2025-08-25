pub mod loading;
pub mod player_states;

use bevy::prelude::*;

use crate::{
    core::systems::despawn_screen, debug::{self, DebugModeState}, game::{
        ui::{self, pause_menu::PauseButtonAction, setting_ui::PauseSettingMenuState, talk::text_window_system::{generate_text_window, handle_talk_event, TalkEvent}},
        world::player,
    }, states::in_game::player_states::{ActionStates, JumpState, MoveStates}, GameState
};

#[derive(Component)]
pub struct OnInGameScreen;

#[derive(Component)]
pub struct InGameEntityMarker;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    #[default]
    Disabled,
    Loading,
    Playing,
    Paused,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseState {
    #[default]
    Disabled,
    InGameMenu,
    PauseMenu,
}

fn start_game(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Loading);
}

pub fn in_game_plugin(app: &mut App) {
    app.init_state::<InGameState>()
        .init_state::<PauseState>()
        .init_state::<ActionStates>()
        .init_state::<MoveStates>()
        .init_state::<JumpState>()
        .init_state::<DebugModeState>()
        .init_state::<PauseButtonAction>()
        .init_state::<PauseSettingMenuState>()
        .add_event::<TalkEvent>()
        .add_plugins((
            loading::loading_plugin,
            ui::in_game_ui_plugin,
            debug::debug_plungin,
        ))
        .add_systems(OnEnter(GameState::InGame), start_game)
        .add_systems(
            OnExit(GameState::InGame),
            despawn_screen::<InGameEntityMarker>,
        )
        .add_observer(generate_text_window)
        .add_systems(Update, (
            handle_talk_event,
        ).run_if(in_state(GameState::InGame)));
}
