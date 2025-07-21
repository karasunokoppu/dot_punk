use bevy::prelude::*;

use crate::in_game::world::{Position, SpriteData};

#[derive(Resource)]
pub struct Player {
    pub name: String,
    pub sprite: SpriteData,
    pub position: Position,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Player".to_string(),
            sprite: SpriteData {
                z_index: 10, //10..19
                image: "default_player_image.png".to_string(),
            },
            //TODO [移動するたびにプレイヤーの座標を更新する]
            position: Position { x: 0.0, y: 0.0 },
        }
    }
}
