use crate::{
    GameState,
    core::{
        components::Position,
        resource::ActiveDatas,
        ui::style::{BACK_GROUND_COLOR, NORMAL_BUTTON, TEXT_COLOR},
    },
    states::main_menu::MenuState,
};
use bevy::prelude::*;

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
pub struct OnMainMenuScreen;

// Actions that can be triggered from a button click
#[derive(Component)]
pub enum MenuButtonAction {
    NewPlay,
    ContinuePlay,
    Settings,
    BackToMainMenu,
    Quit,
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let right_icon = asset_server.load("main_menu/right.png");
    let wrench_icon = asset_server.load("main_menu/wrench.png");
    let exit_icon = asset_server.load("main_menu/exitRight.png");

    commands
        .spawn((
            //1. Background
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            OnMainMenuScreen,
            BackgroundColor(BACK_GROUND_COLOR),
        ))
        .with_children(|parent| {
            // 2. Game Title
            parent.spawn((
                Node {
                    height: Val::Percent(20.0),
                    margin: UiRect {
                        top: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                },
                Text::new("Bevy Game Menu UI"),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // 2. Main Menu Buttons
            // - new game
            // - continue
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
                        MenuButtonAction::NewPlay,
                        children![
                            (ImageNode::new(right_icon.clone()), button_icon_node.clone()),
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
                        MenuButtonAction::ContinuePlay,
                        children![
                            (ImageNode::new(right_icon), button_icon_node.clone()),
                            (
                                Text::new("Continue"),
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
                ],
            ));
        });
}

pub fn main_menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut r_active_datas: ResMut<ActiveDatas>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                //TODO [New GameでInGameに入ったときにActiveDatasとInWorldTimeを更新する]
                MenuButtonAction::NewPlay => {
                    game_state.set(GameState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                //TODO [Continue PlayでSave Dataを選択肢、ActiveDatasとInWorldTimeを更新する]
                MenuButtonAction::ContinuePlay => {
                    r_active_datas.active_stage_id = 1;
                    r_active_datas.teleport_stage = 1;
                    r_active_datas.teleport_position = Position::default();
                    println!("Push ContinuePlay");
                    game_state.set(GameState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}
