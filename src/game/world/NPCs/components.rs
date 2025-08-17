use bevy::prelude::*;

use crate::{core::{components::{Position, SpriteData}, resource::TalkDialogs}, game::world::{player::{components::Direction, states_components::EntityStates}, NPCs::talk::TalkDialog}};

#[derive(Component)]
pub struct NPCMarker;

#[derive(Component)]
pub enum NPCType {
    Merchant,
    QuestGiver,
    Generic,
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
    // 会話
    pub talk_dialog_id: u32, //TODO [クエストの進行状況に応じてTalkDialogのIDを変更する]
}
impl Default for NPC {
    fn default() -> Self {
        NPC {
            id: 0,
            name: "Default NPC".to_string(),
            npm_type: NPCType::Generic,
            sprite: SpriteData {
                z_index: 10, //10..19
                image: "default_npc_image.png".to_string(),
            },
            position: Position::default(),
            direction: Direction::default(),
            states: EntityStates::default(),
            talk_dialog_id: 0,
        }
    }
}