use bevy::prelude::*;

// key settings
#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
pub struct KeyMap {
    pub toggle_pause_menu: KeyCode,
    pub toggle_in_game_menu: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
}

impl Default for KeyMap {
    fn default() -> Self {
        KeyMap {
            // pause menu toggle key( only works when the game is playing )
            toggle_pause_menu: KeyCode::Escape,
            // in-game menu toggle key( only works when the game is playing )
            toggle_in_game_menu: KeyCode::KeyE,
            // movement keys
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
        }
    }
}
