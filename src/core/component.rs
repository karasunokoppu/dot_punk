use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct SpriteData {
    pub z_index: i32,
    pub image: String,
}