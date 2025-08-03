use bevy::prelude::*;

use crate::core::component::SpriteData;
use crate::core::component::Position;

#[derive(Resource)]
pub struct Player {
    pub name: String,
    pub sprite: SpriteData,
    pub position: Position,
    pub move_states: MoveStates,
    pub direction: Direction,
}

#[derive(Default)]
pub enum MoveStates {
    #[default]
    Stand,
    Run,
    Jump,
}

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

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".to_string(),
            sprite: SpriteData {
                z_index: 10, //10..19
                image: "default_player_image.png".to_string(),
            },
            //TODO [更新プログラム作成]
            position: Position::default(),
            //TODO [更新プログラム作成]
            move_states: MoveStates::default(),
            //TODO [更新プログラム作成]
            direction: Direction::default(),
        }
    }
}
