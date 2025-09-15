pub mod loading;
pub mod player_states;

use bevy::prelude::*;

use crate::{
    core::systems::despawn_screen, debug::{self, DebugModeState}, game::{
        ui::{self, pause_menu::PauseButtonAction, setting_ui::PauseSettingMenuState, talk}, world::in_world_time,
    }, states::in_game::player_states::{ActionStates, MoveStates}, GameState
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
        .init_state::<DebugModeState>()
        .init_state::<PauseButtonAction>()
        .init_state::<PauseSettingMenuState>()
        .add_plugins((
            loading::loading_plugin,
            ui::in_game_ui_plugin,
            talk::talk_ui_plugin,
            debug::debug_plungin,
            in_world_time::in_world_time_plugin,
        ))
        .add_systems(OnEnter(GameState::InGame), start_game)
        .add_systems(
            OnExit(GameState::InGame),
            despawn_screen::<InGameEntityMarker>,
        );
}
