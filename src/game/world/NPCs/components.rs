use bevy::prelude::*;

use crate::{core::{components::{Position, SpriteData}}, game::world::{player::{components::Direction, states_components::EntityStates}, NPCs::generic}};

#[derive(Component)]
pub struct NPCMarker;

#[derive(Component)]
pub enum NPCType {
    Merchant,
    QuestGiver,
    Generic(generic::talk::TalkDialog),//TODO [話しかけるシステムを実装する]
}

#[derive(Component)]
pub struct NPC {
    pub id: u32, // Unique identifier for the NPC
    pub name: String,
    pub npm_type: NPCType,
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
            npm_type: NPCType::Generic(generic::talk::TalkDialog::default()),
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