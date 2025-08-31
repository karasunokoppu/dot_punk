pub mod controll;
pub mod senser;

use bevy::prelude::*;

use crate::{game::world::player::interact_entity::{controll::send_interact_event, senser::detect_nearby_activate_entity}, states::in_game::InGameState};

pub fn activate_entity_plugin(app: &mut App) {
    app
    .add_event::<controll::TalkToNPCEvent>()
    .add_systems(Update, (
        detect_nearby_activate_entity,
        send_interact_event,
    ).run_if(in_state(InGameState::Playing)));
}

#[derive(Component, PartialEq, Eq, Clone)]
pub enum InteractEntities{
    NPC(NPCMarker),
    None,
}

#[derive(Component, PartialEq, Eq, Clone)]
pub struct NPCMarker{
    pub id: u32,//NPCのID
}