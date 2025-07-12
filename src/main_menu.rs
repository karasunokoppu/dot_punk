pub mod button;
pub mod main_ui;
pub mod setting_ui;
pub mod style;

use bevy::prelude::*;

use crate::{
    GameState, despawn_screen,
    main_menu::{
        button::button_system,
        main_ui::{MenuButtonAction, OnMainMenuScreen},
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
        .add_systems(OnEnter(GameState::MainMenu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_ui::main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::MainMenu)),
        );
}

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}
