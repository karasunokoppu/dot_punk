mod in_game;
mod main_menu;
mod save_system;
mod setting;
mod splash;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((splash::splash_plugin, main_menu::menu_plugin))
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
