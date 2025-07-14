use bevy::prelude::*;

use crate::{despawn_screen, in_game::{world::{map::components::Maps, ActiveDatas}, InGameState}, utils::style::TEXT_COLOR, GameState};

pub fn loading_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGameState::Loading), (
        loading_setup,
        set_game_stage,
    ).chain())
    .add_systems(OnExit(InGameState::Loading), despawn_screen::<OnLoadingScreen>);
}

#[derive(Component)]
pub struct OnLoadingScreen;

fn loading_setup(mut commands: Commands) {
    // Setup the loading screen UI here
    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
        ZIndex(100),
        OnLoadingScreen,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Loading..."),
            TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
            Node {
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                ..default()
            },
        ));
    });
}

fn set_game_stage(
    mut commands: Commands,
    r_active_datas: Res<ActiveDatas>,
    maps: Res<Maps>,
    assets_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<InGameState>>,
) {
    for map in &maps.map_list {
        if map.id == r_active_datas.active_map_id {
            // Spawn the map or any other entities needed for the game
            let map_image = assets_server.load(&map.sprites.image);
            commands.spawn((
                ImageNode::new(map_image),
                ZIndex(map.sprites.z_index),
            ));
            break;
        }
    }
    // Change the game state to InGame
    game_state.set(InGameState::Playing);
}