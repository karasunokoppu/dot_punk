pub mod components;
pub mod teleport_node;

use bevy::prelude::*;

use crate::{
    core::resource::ActiveDatas, game::world::{
        map::components::{Map, Maps},
        stage::stage001::register_stage001,
    }, GameState
};

pub fn map_plugin(app: &mut App) {
    // Initialize when the game starts
    app.insert_resource(Maps {
        map_list: vec![Map { ..default() }],
    })
    .insert_resource(ActiveDatas {
        active_map_id: 0,
        ..default()
    })
    .add_systems(
        OnEnter(GameState::Splash),
        (
            //Load map data
            register_stage001,
        ),
    );
}
