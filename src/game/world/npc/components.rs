use bevy::prelude::*;

use crate::{
    core::components::{Position, SpriteData},
    game::{ui::talk::components::TalkDialog, world::{
        player::{components::Direction, states_components::EntityStates},
    }},
};

#[derive(Component, PartialEq, Eq)]
pub enum NPCType {
    Merchant,
    QuestGiver,
    Generic(TalkDialog),
}

#[derive(Component)]
pub struct NPC {
    pub id: u32, // Unique identifier for the NPC
    pub name: String,
    pub npc_type: NPCType,
    pub states: EntityStates,
    // 物理的な状態
    pub sprite: SpriteData,
    pub position: Position,
    pub direction: Direction,
}
impl Default for NPC {
    fn default() -> Self {
        NPC {
            id: 0,
            name: "Default NPC".to_string(),
            npc_type: NPCType::Generic(TalkDialog::default()),
            sprite: SpriteData {
                z_index: 10, //10..19
                image: "default_npc_image.png".to_string(),
            },
            position: Position::default(),
            direction: Direction::default(),
            states: EntityStates::default(),
        }
    }
}
impl NPC {
    pub fn new(id: u32) -> Self {
        NPC {
            id: id,
            name: format!("Citizen {}", id),
            npc_type: NPCType::Generic(TalkDialog::new(id)),
            sprite: SpriteData {
                z_index: 10, //10..19
                image: "default_npc_image.png".to_string(),
            },
            position: Position::default(),
            direction: Direction::default(),
            states: EntityStates::default(),
        }
    }
    pub fn with_position(self: Self, position: Position) -> Self {
        NPC {
            id: self.id,
            name: self.name,
            npc_type: self.npc_type,
            sprite: self.sprite,
            position: position,
            direction: self.direction,
            states: self.states,
        }
    }
}
