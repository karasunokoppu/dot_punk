pub mod main_ui;
pub mod setting_ui;

use bevy::prelude::*;

use crate::{
    GameState,
    core::ui::button::button_system,
    core::systems::despawn_screen,
    states::main_menu::{
        main_ui::OnMainMenuScreen,
        setting_ui::{MainSettingMenuState, OnSettingsMenuScreen},
    },
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .init_state::<MainSettingMenuState>()
        //Main Menu
        .add_systems(OnEnter(GameState::MainMenu), menu_setup)
        //Main Menu
        .add_systems(
            Update,
            (button_system).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnEnter(MenuState::Main), main_ui::main_menu_setup)
        .add_systems(
            Update,
            (main_ui::main_menu_action,).run_if(in_state(MenuState::Main)),
        )
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        //Setting Menu
        .add_systems(
            OnEnter(MenuState::Settings),
            setting_ui::main_setting_menu_setup,
        )
        .add_systems(
            Update,
            (
                setting_ui::setting_menu_action,
                // setting_ui::swap_setting_menu,
            )
                .run_if(in_state(MenuState::Settings)),
        )
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        );
}

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}
