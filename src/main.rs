mod core;
mod debug;
mod game;
mod states;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::core::setting::key_map;

const WINDOW_WIDTH: f32 = 1200.;
const WINDOW_HIGHT: f32 = 800.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WIDTH, WINDOW_HIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity::ZERO)
        .init_resource::<key_map::KeyMap>()
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((
            states::splash::splash_plugin,
            states::main_menu::menu_plugin,
            states::in_game::in_game_plugin,
        ))
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    InGame,
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MainCamera, Transform::from_xyz(0.0, 0.0, 0.0)));
}

//TODO [bevy_light_2dを試す]
