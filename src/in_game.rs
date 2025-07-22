pub mod loading;
pub mod ui;
pub mod world;

use bevy::prelude::*;

use crate::{
    GameState, despawn_screen,
    in_game::world::{InGameEntityMarker, player},
};

#[derive(Component)]
pub struct OnInGameScreen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InGameState {
    #[default]
    Disabled,
    Loading,
    Playing,
    Paused,
}

fn start_game(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Loading);
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseState {
    #[default]
    Disabled,
    InGameMenu,
    PauseMenu,
}

pub fn in_game_plugin(app: &mut App) {
    app.init_state::<InGameState>()
        .init_state::<PauseState>()
        .add_plugins(loading::loading_plugin)
        .add_plugins(ui::in_game_ui_plugin)
        .add_plugins(player::player_plugin)
        .add_systems(OnEnter(GameState::InGame), start_game)
        .add_systems(
            OnExit(GameState::InGame),
            despawn_screen::<InGameEntityMarker>,
        )
        // for debugging states
        .add_systems(OnEnter(InGameState::Playing), || {
            println!(" > InGameState::Playing")
        })
        .add_systems(OnEnter(InGameState::Paused), || {
            println!(" > InGameState::Paused")
        })
        .add_systems(OnEnter(PauseState::Disabled), || {
            println!(" > PauseState::Disabled")
        })
        .add_systems(OnEnter(PauseState::InGameMenu), || {
            println!(" > PauseState::InGameMenu")
        })
        .add_systems(OnEnter(PauseState::PauseMenu), || {
            println!(" > PauseState::PauseMenu")
        });
}
