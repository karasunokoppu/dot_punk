pub mod map;
pub mod player;
pub mod stage;

use crate::core::component::Position;
use bevy::prelude::*;

#[derive(Component)]
pub struct InGameEntityMarker;

#[derive(Resource, Default)]
pub struct ActiveDatas {
    pub active_map_id: u32,
    pub teleport_map: u32,           // Next teleport map ID
    pub teleport_position: Position, // Next teleport node ID
}



#[derive(Component, Default)]
pub struct SpriteData {
    pub z_index: i32,
    pub image: String,
}
