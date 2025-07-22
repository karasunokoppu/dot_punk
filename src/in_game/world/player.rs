pub mod components;
pub mod controll;

use bevy::prelude::*;

use crate::in_game::{InGameState, world::map::teleport_node::detect_teleport_node_colliding};

pub fn player_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            controll::move_player,
            //テレポートノードに接した際などに衝突を検出するため
            detect_teleport_node_colliding,
        )
            .run_if(in_state(InGameState::Playing)),
    );
}
