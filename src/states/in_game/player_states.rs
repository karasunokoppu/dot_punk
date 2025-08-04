use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ActionStates {
    #[default]
    Disable,
    Stand,
    Move,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MoveStates {
    #[default]
    Wark,
    Run,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum JumpState {
    #[default]
    NotJump,
    FirstJump,
    SecondJump,
}
