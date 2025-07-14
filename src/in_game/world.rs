pub mod map;
pub mod player;
pub mod stage;

use bevy::prelude::*;

#[derive(Resource)]
pub struct ActiveDatas {
    pub active_map_id: u32,
}

#[derive(Component, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct SpriteData {
    pub z_index: i32,
    pub image: String,
}