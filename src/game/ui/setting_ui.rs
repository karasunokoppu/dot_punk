use bevy::prelude::*;

use crate::{
    core::ui::style::{BACK_GROUND_COLOR, NORMAL_BUTTON, TEXT_COLOR},
    game::ui::pause_menu::PauseButtonAction, states::in_game::InGameState,
};

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnPauseSettingsMenuScreen;
#[derive(Component)]
pub struct OnSettingsMenu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
pub enum PauseSettingMenuState {
    #[default]
    Disabled,
    Display,
    Sound,
    KeyBind,
    Interface,
}

#[derive(Component)]
pub enum PauseSettingMenuSideBarAction {
    Display,
    Sound,
    KeyBind,
    Interface,
    Back,
}

pub fn pause_setting_menu_setup(mut commands: Commands) {
    // Common style for Side Bar buttons on the screen
    let side_button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let side_button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };

    commands
        .spawn((
            // 1. Background
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(BACK_GROUND_COLOR),
            OnPauseSettingsMenuScreen,
        ))
        .with_children(|parent| {
            //2. Side Bar
            parent.spawn((
                Node {
                    width: Val::Percent(30.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                BackgroundColor(Color::BLACK.with_alpha(0.5)),
                children![
                    (
                        Button,
                        side_button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        PauseSettingMenuSideBarAction::Display,
                        children![
                            // (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
                            (
                                Text::new("Display"),
                                side_button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ],
                    ),
                    (
                        Button,
                        side_button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        PauseSettingMenuSideBarAction::Sound,
                        children![
                            // (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
                            (
                                Text::new("Sound"),
                                side_button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ],
                    ),
                    (
                        Button,
                        side_button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        PauseSettingMenuSideBarAction::KeyBind,
                        children![
                            // (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
                            (
                                Text::new("Key Bind"),
                                side_button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ],
                    ),
                    (
                        Button,
                        side_button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        PauseSettingMenuSideBarAction::Interface,
                        children![
                            // (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
                            (
                                Text::new("Interface"),
                                side_button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ],
                    ),
                    (
                        Button,
                        side_button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        PauseSettingMenuSideBarAction::Back,
                        children![
                            // (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
                            (
                                Text::new("Back"),
                                side_button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ],
                    )
                ],
            ));
            //2. Setting Menus
            parent
                .spawn((
                    Node {
                        width: Val::Percent(70.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    OnSettingsMenu,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::None,
                            ..default()
                        },
                        BackgroundColor(Color::BLACK),
                        PauseSettingMenuState::Display,
                    )); //TODO [Display設定UIを実装]
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::None,
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        PauseSettingMenuState::Sound,
                    )); //TODO [Sound設定UIを実装]
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::None,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                        PauseSettingMenuState::KeyBind,
                    )); //TODO [KeyBind設定UIを実装]
                    parent.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            display: Display::None,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
                        PauseSettingMenuState::Interface,
                    )); //TODO [Interface設定UIを実装]
                });
        });
}

pub fn setting_menu_action(
    interaction_query: Query<
        (&Interaction, &PauseSettingMenuSideBarAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_pause_buttun_state: ResMut<NextState<PauseButtonAction>>,
    mut pause_setting_menu_state: ResMut<NextState<PauseSettingMenuState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
    mut target_bundle: Query<(&mut Node, &PauseSettingMenuState)>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PauseSettingMenuSideBarAction::Display => {
                    swap_setting_menu(PauseSettingMenuState::Display, &mut target_bundle);
                    pause_setting_menu_state.set(PauseSettingMenuState::Display);
                }
                PauseSettingMenuSideBarAction::Sound => {
                    swap_setting_menu(PauseSettingMenuState::Sound, &mut target_bundle);
                    pause_setting_menu_state.set(PauseSettingMenuState::Sound);
                }
                PauseSettingMenuSideBarAction::KeyBind => {
                    swap_setting_menu(PauseSettingMenuState::KeyBind, &mut target_bundle);
                    pause_setting_menu_state.set(PauseSettingMenuState::KeyBind);
                }
                PauseSettingMenuSideBarAction::Interface => {
                    swap_setting_menu(PauseSettingMenuState::Interface, &mut target_bundle);
                    pause_setting_menu_state.set(PauseSettingMenuState::Interface);
                }
                PauseSettingMenuSideBarAction::Back => {
                    pause_setting_menu_state.set(PauseSettingMenuState::Disabled);
                    next_pause_buttun_state.set(PauseButtonAction::Disabled);
                    next_in_game_state.set(InGameState::Playing);
                }
            }
        }
    }
}

pub fn swap_setting_menu(
    activated_setting: PauseSettingMenuState,
    target_bundle: &mut Query<(&mut Node, &PauseSettingMenuState)>,
) {
    for (mut node, pause_setting_menu_state) in target_bundle {
        if *pause_setting_menu_state == activated_setting {
            node.display = Display::Block
        } else {
            node.display = Display::None
        }
    }
}
