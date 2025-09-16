use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct SpriteData {
    pub z_index: i32,
    pub image: String,
}
