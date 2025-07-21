use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    GameState, despawn_screen,
    in_game::{
        InGameState,
        world::{
            ActiveDatas, InGameEntityMarker,
            map::{
                self,
                components::{Maps, PlayerMarker, TeleportNode, TeleportNodeMarker},
            },
        },
    },
    utils::style::TEXT_COLOR,
};

pub fn loading_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(InGameState::Loading),
        (
            loading_setup,
            despawn_screen::<InGameEntityMarker>,
            set_game_stage,
        )
            .chain(),
    )
    .add_systems(
        OnExit(InGameState::Loading),
        despawn_screen::<OnLoadingScreen>,
    );
}

#[derive(Component)]
pub struct OnLoadingScreen;

fn loading_setup(mut commands: Commands) {
    // Setup the loading screen UI here
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
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
        //spawn map sprites
        if map.id == r_active_datas.teleport_map {
            // Spawn sprites (the map or any other entities needed for the game stage)
            for sprite in &map.sprites {
                let map_image = assets_server.load(&sprite.image);
                commands.spawn((
                    InGameEntityMarker,
                    Sprite::from_image(map_image),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    ZIndex(sprite.z_index),
                ));
            }
            for wall_collider in &map.wall_colliders {
                commands.spawn((
                    InGameEntityMarker,
                    RigidBody::Static,
                    Collider::segment(
                        Vec2::new(wall_collider.start_node.x, wall_collider.start_node.y),
                        Vec2::new(wall_collider.end_node.x, wall_collider.end_node.y),
                    ),
                ));
            }
            // Spawn teleport nodes
            for teleport_node in &map.teleport_nodes {
                commands.spawn((
                    InGameEntityMarker,
                    Transform::from_xyz(
                        teleport_node.node_position.x,
                        teleport_node.node_position.y,
                        0.0,
                    ),
                    Collider::circle(20.0),
                    RigidBody::Static,
                    TeleportNodeMarker,
                    CollidingEntities::default(),
                    TeleportNode {
                        id: teleport_node.id,
                        node_position: teleport_node.node_position,
                        target_map: teleport_node.target_map,
                        teleport_position: teleport_node.teleport_position,
                    },
                ));
            }
        }
        //spawn player
        if map.id == r_active_datas.teleport_map {
            commands.spawn((
                InGameEntityMarker,
                PlayerMarker,
                RigidBody::Dynamic,
                Collider::circle(20.0),
                LockedAxes::ROTATION_LOCKED,
                CollidingEntities::default(),
                Transform::from_xyz(
                    r_active_datas.teleport_position.x,
                    r_active_datas.teleport_position.y,
                    0.0,
                ),
            ));
        }
    }
    // Change the game state to InGame
    game_state.set(InGameState::Playing);
}
