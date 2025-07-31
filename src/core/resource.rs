use crate::core::component::Position;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ActiveDatas {
    pub active_map_id: u32,
    pub teleport_map: u32,           // Next teleport map ID
    pub teleport_position: Position, // Next teleport node ID
}