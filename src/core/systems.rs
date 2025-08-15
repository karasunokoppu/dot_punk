use bevy::prelude::*;
use std::any::type_name;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn state_change_detect<T: States + Copy>(
    game_states: Res<State<T>>,
    mut last_state: Local<Option<T>>,
) {
    if Some(game_states.get().clone()) != *last_state {
        *last_state = Some(game_states.get().clone());
        let current_state = format!("{:?}", last_state.unwrap());

        let mut t_type = type_name::<T>();
        if let Some(pos) = t_type.rfind("::") {
            t_type = &t_type[pos + 2..];
        }
        println!("{} {} {} {}", ">", t_type, "changed to", current_state,);
    }
}
