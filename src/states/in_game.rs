pub mod loading;
pub mod player_states;

use bevy::prelude::*;

use crate::{
    GameState, core::systems::despawn_screen,
    game::{ui, world::player},
    states::in_game::player_states::{ActionStates, JumpState, MoveStates},
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
        .add_plugins((loading::loading_plugin, ui::in_game_ui_plugin))
        .add_systems(OnEnter(GameState::InGame), start_game)
        .add_systems(
            OnExit(GameState::InGame),
            despawn_screen::<InGameEntityMarker>,
        );
}
