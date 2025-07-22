use bevy::prelude::*;

use crate::{main_menu::MenuState, utils::style::{BACK_GROUND_COLOR, NORMAL_BUTTON, TEXT_COLOR}};

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnSettingsMenuScreen;
#[derive(Component)]
pub struct OnSettingsMenu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
pub enum MainSettingMenuState {
    #[default]
    Disabled,
    Display,
    Sound,
    KeyBind,
    Interface,
}

#[derive(Component)]
pub enum MainSettingMenuSideBarAction {
    Display,
    Sound,
    KeyBind,
    Interface,
    Back,
}
//TODO [設定UIを作成。ゲーム内設定も実装]

pub fn main_setting_menu_setup(mut commands: Commands) {
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
            OnSettingsMenuScreen,
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
                        MainSettingMenuSideBarAction::Display,
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
                        MainSettingMenuSideBarAction::Sound,
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
                        MainSettingMenuSideBarAction::KeyBind,
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
                        MainSettingMenuSideBarAction::Interface,
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
                        MainSettingMenuSideBarAction::Back,
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
            parent.spawn((
                Node {
                    width: Val::Percent(70.),
                    height: Val::Percent(100.),
                    ..default()
                },
                OnSettingsMenu,
            )).with_children(|parent| {
                parent.spawn((
                    Node{
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        display: Display::None,
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                    MainSettingMenuState::Display,
                ));//TODO [Display設定UIを実装]
                parent.spawn((
                    Node{
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        display: Display::None,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                    MainSettingMenuState::Sound,
                ));//TODO [Sound設定UIを実装]
                parent.spawn((
                    Node{
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        display: Display::None,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                    MainSettingMenuState::KeyBind,
                ));//TODO [KeyBind設定UIを実装]
                parent.spawn((
                    Node{
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        display: Display::None,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
                    MainSettingMenuState::Interface,
                ));//TODO [Interface設定UIを実装]
            });
        });
}

pub fn setting_menu_action(
    interaction_query: Query<
        (&Interaction, &MainSettingMenuSideBarAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut main_setting_menu_state: ResMut<NextState<MainSettingMenuState>>,
    mut target_bundle: Query<(&mut Node, &MainSettingMenuState)>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainSettingMenuSideBarAction::Display => {
                    swap_setting_menu(MainSettingMenuState::Display, &mut target_bundle);
                    main_setting_menu_state.set(MainSettingMenuState::Display);
                }
                MainSettingMenuSideBarAction::Sound => {
                    swap_setting_menu(MainSettingMenuState::Sound, &mut target_bundle);
                    main_setting_menu_state.set(MainSettingMenuState::Sound);
                }
                MainSettingMenuSideBarAction::KeyBind => {
                    swap_setting_menu(MainSettingMenuState::KeyBind, &mut target_bundle);
                    main_setting_menu_state.set(MainSettingMenuState::KeyBind);
                }
                MainSettingMenuSideBarAction::Interface => {
                    swap_setting_menu(MainSettingMenuState::Interface, &mut target_bundle);
                    main_setting_menu_state.set(MainSettingMenuState::Interface);
                }
                MainSettingMenuSideBarAction::Back => {
                    main_setting_menu_state.set(MainSettingMenuState::Disabled);
                    next_menu_state.set(MenuState::Main);
                }
            }
        }
    }
}

pub fn swap_setting_menu(
    activated_setting: MainSettingMenuState,
    target_bundle: &mut  Query<(&mut Node, &MainSettingMenuState)>
) {
    for (mut node, main_setting_menu_state) in target_bundle {
        if *main_setting_menu_state == activated_setting {
            node.display = Display::Block
        }else {
            node.display = Display::None
        }
    }
}