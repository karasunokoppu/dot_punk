use crate::{
    core::{components::{Position, SpriteData}, setting::game_setting::{MOST_DARKER_BRIGHTNESS, ONE_DAY_HOUR, ONE_HOUR_MINUTE}},
    game::world::{
        player::{components::Direction, interact_entity::{InteractEntities, NPCMarker}, states_components::EntityStates},
        stage::component::Stage,
    },
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct ActiveDatas {
    pub active_stage_id: u32,
    pub active_stage_name: String,
    pub teleport_stage: u32,         // Next teleport map ID
    pub teleport_position: Position, // Next teleport node ID
    pub closest_interact_entity_type: InteractEntities,
    pub talking_npc: Option<u32>,
    pub talk_index: Option<u32>
}
impl Default for ActiveDatas {
    fn default() -> Self {
        ActiveDatas {
            active_stage_id: 0,
            active_stage_name: "Default Map".to_string(),
            teleport_stage: 0,
            teleport_position: Position {
                x: -50.0,
                y: -100.0,
            },
            closest_interact_entity_type: InteractEntities::NPC(NPCMarker { id: 0}),
            talking_npc: None,
            talk_index: None,
        }
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct InWorldTime{
    pub hour: i32,
    pub minute: i32,
}
impl InWorldTime {
    pub fn default() -> Self {
        InWorldTime {
            hour: 12,
            minute: 0,
        }
    }

    pub fn add_time(self: &Self, hour: i32, minute: i32) -> Self{
        let added_minute = self.minute + minute;
        let add_hour = added_minute / ONE_HOUR_MINUTE;
        let new_minute = added_minute % ONE_HOUR_MINUTE;
        let added_hour = self.hour + hour + add_hour;
        let new_hour = added_hour % ONE_DAY_HOUR;
        InWorldTime {
            hour: new_hour,
            minute: new_minute
        }
    }

    pub fn get_brightness_from_time(self: &Self) -> f32{
        let current_time = self.hour * ONE_HOUR_MINUTE + self.minute;
        let all_minute = ONE_DAY_HOUR * ONE_HOUR_MINUTE;
        let brightness: f32 = current_time as f32 / (all_minute / 2) as f32 - 1.0;
        if brightness.abs() <= MOST_DARKER_BRIGHTNESS {
            brightness.abs()
        }else{
            MOST_DARKER_BRIGHTNESS
        }
    }
}

#[derive(Resource)]
pub struct Stages {
    pub stage_list: Vec<Stage>,
}
impl Default for Stages {
    fn default() -> Self {
        Stages {
            stage_list: vec![Stage::default()],
        }
    }
}

#[derive(Resource)]
pub struct Player {
    pub name: String,
    pub sprite: SpriteData,
    pub position: Position,
    pub direction: Direction,
    pub states: EntityStates,
}
impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player Charactor01".to_string(),
            sprite: SpriteData {
                z_index: 10, //10..19
                image: "default_player_image.png".to_string(),
            },
            position: Position::default(),
            direction: Direction::default(),
            //TODO [更新プログラム作成]
            states: EntityStates::default(),
        }
    }
}
