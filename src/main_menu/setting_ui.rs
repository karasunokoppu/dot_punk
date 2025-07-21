use bevy::prelude::*;

use crate::utils::style::{BACK_GROUND_COLOR, NORMAL_BUTTON, TEXT_COLOR};

// Tag component used to tag entities added on the settings menu screen
#[derive(Component)]
pub struct OnSettingsMenuScreen;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
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
    Quit,
}
//TODO [設定UIを作成。ゲーム内設定も実装]

pub fn main_setting_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        MainSettingMenuSideBarAction::Quit,
                        children![
                            // (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
                            (
                                Text::new("Quit"),
                                side_button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ],
                    )
                ],
            ));
            //2. Setting Menus
            parent.spawn((Node {
                width: Val::Percent(70.),
                height: Val::Percent(100.),
                ..default()
            },));
        });
}

pub fn setting_menu_action(
    interaction_query: Query<
        (&Interaction, &MainSettingMenuSideBarAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                //TODO [処理を実装]
                MainSettingMenuSideBarAction::Display => {}
                MainSettingMenuSideBarAction::Sound => {}
                MainSettingMenuSideBarAction::KeyBind => {}
                MainSettingMenuSideBarAction::Interface => {}
                MainSettingMenuSideBarAction::Quit => {}
            }
        }
    }
}