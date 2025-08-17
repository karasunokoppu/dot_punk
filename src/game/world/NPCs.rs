pub mod talk;
pub mod components;

use bevy::prelude::*;

use crate::{core::resource::TalkDialogs, game::world::NPCs::talk::TalkDialog};

pub fn npc_plugin(app: &mut App) {
    app.insert_resource(TalkDialogs{
        dialog_list: vec![
            TalkDialog::default(),
        ]
    });
}


