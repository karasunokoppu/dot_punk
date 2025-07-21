use bevy::prelude::*;

use crate::{
    GameState,
    in_game::{InGameState, PauseState},
    setting::key_map,
    utils::style::{NORMAL_BUTTON, TEXT_COLOR},
};

#[derive(Component)]
pub struct OnPauseMenuScreen;

#[derive(Component)]
pub enum PauseButtonAction {
    Save,
    Load,
    Settings,
    MainMenu,
    Quit,
}

pub fn toggle_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<OnPauseMenuScreen>>,
    key_maps: Res<key_map::KeyMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // InGameState
    in_game_state: Res<State<InGameState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    // PauseState
    pause_menu: Res<State<PauseState>>,
    mut next_pause_menu_state: ResMut<NextState<PauseState>>,
) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    if keyboard_input.just_pressed(key_maps.toggle_pause_menu) {
        match (in_game_state.get(), pause_menu.get()) {
            // If Pause Menu is open, close it
            (InGameState::Paused, PauseState::PauseMenu) => {
                if !query.is_empty() {
                    for entity in query.iter() {
                        commands.entity(entity).despawn();
                    }
                }
                // change the game state to playing and pause state to disabled
                next_pause_menu_state.set(PauseState::Disabled);
                next_in_game_state.set(InGameState::Playing);
            }
            // If the game is playing, we can open the pause menu
            (InGameState::Playing, PauseState::Disabled) => {
                // Otherwise, we can spawn the pause menu UI
                commands
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
                        OnPauseMenuScreen,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Node {
                                    width: Val::Percent(30.0),
                                    height: Val::Percent(100.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                BackgroundColor(Color::BLACK.with_alpha(0.7)),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text("Pause Menu".to_string()),
                                    TextFont {
                                        font_size: 67.0,
                                        ..default()
                                    },
                                    TextColor(TEXT_COLOR),
                                ));
                                // Display three buttons for each action available from the main menu:
                                // - new game
                                // - settings
                                // - quit
                                parent.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(70.0),
                                        margin: UiRect {
                                            right: Val::Px(50.0),
                                            top: Val::Px(20.0),
                                            ..Default::default()
                                        },
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::FlexStart,
                                        align_items: AlignItems::FlexEnd,
                                        ..default()
                                    },
                                    children![
                                        (
                                            Button,
                                            button_node.clone(),
                                            BackgroundColor(NORMAL_BUTTON),
                                            PauseButtonAction::Save,
                                            children![
                                                Text::new("Save"),
                                                button_text_font.clone(),
                                                TextColor(TEXT_COLOR),
                                            ],
                                        ),
                                        (
                                            Button,
                                            button_node.clone(),
                                            BackgroundColor(NORMAL_BUTTON),
                                            PauseButtonAction::Load,
                                            children![
                                                Text::new("Load"),
                                                button_text_font.clone(),
                                                TextColor(TEXT_COLOR),
                                            ],
                                        ),
                                        (
                                            Button,
                                            button_node.clone(),
                                            BackgroundColor(NORMAL_BUTTON),
                                            PauseButtonAction::Settings,
                                            children![
                                                Text::new("Settings"),
                                                button_text_font.clone(),
                                                TextColor(TEXT_COLOR),
                                            ]
                                        ),
                                        (
                                            Button,
                                            button_node.clone(),
                                            BackgroundColor(NORMAL_BUTTON),
                                            PauseButtonAction::MainMenu,
                                            children![
                                                Text::new("Main Menu"),
                                                button_text_font.clone(),
                                                TextColor(TEXT_COLOR),
                                            ]
                                        ),
                                        (
                                            Button,
                                            button_node.clone(),
                                            BackgroundColor(NORMAL_BUTTON),
                                            PauseButtonAction::Quit,
                                            children![
                                                Text::new("Quit"),
                                                button_text_font.clone(),
                                                TextColor(TEXT_COLOR),
                                            ]
                                        ),
                                    ],
                                ));
                            });
                    });
                // change the game state to paused and the pause state to pause menu
                next_in_game_state.set(InGameState::Paused);
                next_pause_menu_state.set(PauseState::PauseMenu);
            }
            _ => {
                println!(">! Error: InGameState and PauseState do not match");
            }
        }
    }
}

pub fn pause_menu_action(
    interaction_query: Query<
        (&Interaction, &PauseButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut next_pause_menu_state: ResMut<NextState<PauseState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    //Main Menu
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PauseButtonAction::Save => {
                    //1. save the datas
                    //2. change the state
                    println!(">! Save action triggered");
                }
                PauseButtonAction::Load => {
                    //1. load the datas
                    //2. change the state
                    println!(">! Load action triggered");
                }
                PauseButtonAction::Settings => {
                    //1. change the state
                    println!(">! Settings action triggered");
                }
                PauseButtonAction::MainMenu => {
                    next_in_game_state.set(InGameState::Disabled);
                    next_pause_menu_state.set(PauseState::Disabled);
                    next_game_state.set(GameState::MainMenu);
                    println!(">! Main Menu action triggered");
                }
                PauseButtonAction::Quit => {
                    println!(">! Quit action triggered");
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}
