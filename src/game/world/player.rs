pub mod animation;
pub mod components;
pub mod controll;
pub mod states_components;
pub mod activate_entity;

use bevy::prelude::*;

use crate::core::resource::Player;
use crate::game::world::map::teleport_node::detect_teleport_node_colliding;
use crate::game::world::player::activate_entity::senser::detect_nearby_activate_entity;
use crate::states::in_game::InGameState;
use crate::states::in_game::player_states::ActionStates;

pub fn player_plugin(app: &mut App) {
    app.init_resource::<Player>()
        .add_systems(
            Update,
            (
                controll::move_player,
                //テレポートノードに接した際などに衝突を検出するため
                detect_teleport_node_colliding,
                controll::update_player_pos_resource,
                detect_nearby_activate_entity,
            )
                .run_if(in_state(InGameState::Playing)),
        )
        .add_systems(
            Update,
            (controll::dash_mode,).run_if(in_state(ActionStates::Move)),
        )
        .add_systems(
            Update,
            controll::update_camera_pos.run_if(not(in_state(ActionStates::Disable))),
        );
}
