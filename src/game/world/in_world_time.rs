use bevy::prelude::*;

use crate::{core::{resource::InWorldTime, setting::game_setting::{ONE_DAY_HOUR, ONE_HOUR_MINUTE}}, GameState};

pub fn in_world_time_plugin(app: &mut App){
    app
    .add_systems(OnEnter(GameState::InGame),setup_time_view)
    .add_systems(Update, add_time.run_if(in_state(GameState::InGame)));
}

#[derive(Component)]
pub struct TimeViewMarker;

pub fn setup_time_view(
    mut commands: Commands,
    res_time: Res<InWorldTime>,
){
    let time_to_sun: f32 = res_time.get_brightness_from_time();

    commands.spawn((
        Node{
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        TimeViewMarker,
        BackgroundColor(Color::BLACK.with_alpha(time_to_sun as f32)),
        GlobalZIndex(30),
    ));
}

pub fn add_time(
    mut res_time: ResMut<InWorldTime>,
    mut time_view_colors: Query<(&mut BackgroundColor, &TimeViewMarker)>,
    key_input: Res<ButtonInput<KeyCode>>,
){
    if key_input.just_pressed(KeyCode::ArrowUp) {
        match time_view_colors.single_mut(){
            Ok((mut time_view_color, _)) => {
                let now: InWorldTime = res_time.clone();
                *res_time = now.add_time(0, 30);
                println!("now : {}:{}", res_time.hour, res_time.minute);
                *time_view_color = BackgroundColor(Color::BLACK.with_alpha(res_time.get_brightness_from_time()));
                println!("{}", res_time.get_brightness_from_time());
            }
            Err(e) => {println!("Error {e} is occured!!")}
        }
    }
}