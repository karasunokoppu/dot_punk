use bevy::prelude::*;

use crate::core::components::Position;
use crate::core::components::SpriteData;
use crate::game::world::player::states_components::EntityStates;



#[derive(Default, PartialEq, Debug, Clone)]
pub enum Direction {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    #[default]
    Top,
    Bottom,
    Right,
    Left,
}