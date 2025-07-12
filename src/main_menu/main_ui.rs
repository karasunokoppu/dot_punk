use crate::main_menu::style::{
    NORMAL_BUTTON,
    TEXT_COLOR,
    BACK_GROUND_COLOR,
};
use bevy::prelude::*;

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

// Actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    Quit,
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
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

    let right_icon = asset_server.load("main_menu/right.png");
    let wrench_icon = asset_server.load("main_menu/wrench.png");
    let exit_icon = asset_server.load("main_menu/exitRight.png");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // flex_direction: FlexDirection::Column,
                // align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(BACK_GROUND_COLOR),
            children![
                // Display the game name
                (
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                    Text::new("Bevy Game Menu UI"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                ),
                // Display three buttons for each action available from the main menu:
                // - new game
                // - settings
                // - quit
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ],
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        (ImageNode::new(wrench_icon), button_icon_node.clone()),
                        (
                            Text::new("Settings"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ]
        )],
    ));
}
