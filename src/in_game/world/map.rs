pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::{
    GameState,
    in_game::world::{
        ActiveDatas,
        map::components::{Map, Maps},
        stage::stage001::register_stage001,
    },
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
