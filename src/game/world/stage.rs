pub mod component;
pub mod stage001;
//各ステージのマップやNPCなどの情報を定義する
use bevy::prelude::*;

use crate::core::resource::Stages;

#[derive(Component)]
pub struct StageMarker;

pub fn stage_plugin(app: &mut App) {
    app
    .init_resource::<Stages>()
    .add_systems(Startup, stage001::register_stage001);
}