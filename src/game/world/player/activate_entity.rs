pub mod senser;

use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Clone)]
pub enum ActivateEntities{
    NPC(NPCMarker),
    None,
}

#[derive(Component, PartialEq, Eq, Clone)]
pub struct NPCMarker{
    pub id: u32,
}