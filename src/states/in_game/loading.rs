use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    core::{resource::{ActiveDatas, Stages}, systems::despawn_screen, ui::style::TEXT_COLOR},
    game::world::{map::components::{PlayerMarker, TeleportNode, TeleportNodeMarker}, NPCs::components::NPCMarker},
    states::in_game::{InGameEntityMarker, InGameState},
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
    mut r_active_datas: ResMut<ActiveDatas>,
    stages: Res<Stages>,
    assets_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<InGameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for stage in &stages.stage_list {
        //spawn stage sprites
        if stage.id == r_active_datas.teleport_stage {
            // Spawn sprites (the stages or any other entities needed for the game stage)
            for sprite in &stage.map.sprites {
                let stages_image = assets_server.load(&sprite.image);
                commands.spawn((
                    InGameEntityMarker,
                    Sprite::from_image(stages_image),
                    Transform::from_xyz(0.0, 0.0, sprite.z_index as f32),
                    GlobalZIndex(sprite.z_index),
                ));
            }
            for wall_collider in &stage.map.wall_colliders {
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
            for teleport_node in &stage.map.teleport_nodes {
                commands.spawn((
                    InGameEntityMarker,
                    Transform::from_xyz(
                        teleport_node.node_position.x,
                        teleport_node.node_position.y,
                        10.0,
                    ),
                    Collider::circle(20.0),
                    RigidBody::Static,
                    TeleportNodeMarker,
                    CollidingEntities::default(),
                    TeleportNode {
                        id: teleport_node.id,
                        node_position: teleport_node.node_position,
                        target_stage: teleport_node.target_stage,
                        teleport_position: teleport_node.teleport_position,
                    },
                    GlobalZIndex(10),
                    //for debugging
                    Mesh2d(meshes.add(Circle::new(20.0))),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                ));
            }
            r_active_datas.active_stage_name = stage.name.clone();

            //spawn player
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
                    10.0,
                ),
                GlobalZIndex(10),
                //for debugging
                Mesh2d(meshes.add(Circle::new(20.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 1.0))),
            ));

            //spawn NPCs
            for npc in &stage.npcs {
                commands.spawn((
                    InGameEntityMarker,
                    NPCMarker,
                    RigidBody::Static,
                    Collider::circle(20.0),
                    LockedAxes::ROTATION_LOCKED,
                    CollidingEntities::default(),
                    Transform::from_xyz(npc.position.x, npc.position.y, 10.0),
                    GlobalZIndex(10),
                    //for debugging
                    Mesh2d(meshes.add(Circle::new(20.0))),
                    MeshMaterial2d(materials.add(Color::srgb(0.5, 1.0, 0.5))),
                ));
            }
        }
    }
    // Change the game state to InGame
    game_state.set(InGameState::Playing);
}
