pub mod key_map;
pub mod game_setting;
use bevy::prelude::*;

pub fn setting_plugin(app: &mut App) {
    app.init_resource::<key_map::KeyMap>();
}