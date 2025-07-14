pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::in_game::world::{map::components::{Map, Maps}, ActiveDatas};

pub fn map_plugin(app: &mut App) {
    app
    .insert_resource(Maps {
        map_list: vec![Map{..default()}],
    })
    .insert_resource(ActiveDatas{
        active_map_id: 0,
    });
}