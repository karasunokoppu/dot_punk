mod in_game;
mod main_menu;
mod save_system;
mod setting;
mod splash;
mod utils;

use avian2d::prelude::*;
use bevy::prelude::*;

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
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((
            splash::splash_plugin,
            setting::setting_plugin,
            main_menu::menu_plugin,
            in_game::in_game_plugin,
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

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}
