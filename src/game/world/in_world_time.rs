use std::time::Duration;

use bevy::prelude::*;

use crate::{core::{resource::InWorldTime, setting::game_setting::{ONE_DAY_HOUR, ONE_HOUR_MINUTE, ONE_MINUTE_SECOND}, systems::despawn_screen}, GameState};

pub fn in_world_time_plugin(app: &mut App){
    app
    .add_systems(OnEnter(GameState::InGame),(
        setup_time_view,
        add_timer,
    ))
    .add_systems(Update, (
        add_time_for_debug,
        spend_time,
    ).run_if(in_state(GameState::InGame)))
    .add_systems(OnExit(GameState::InGame), (
        despawn_screen::<TimeViewMarker>,
        despawn_timer,
    ));
}

#[derive(Component)]
pub struct TimeSpendTimer{
    timer: Timer,
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
        BackgroundColor(Color::BLACK.with_alpha(time_to_sun)),
        GlobalZIndex(30),
    ));
}

pub fn spend_time(
    time: Res<Time>,
    mut timer: Query<(Entity, &mut TimeSpendTimer)>,
    mut res_time: ResMut<InWorldTime>,
    mut time_view_colors: Query<(&mut BackgroundColor, &TimeViewMarker)>,
){
    for (_, mut time_spend_timer) in timer.iter_mut(){
        time_spend_timer.timer.tick(time.delta());
        if time_spend_timer.timer.just_finished(){
            *res_time = res_time.add_time(0, 1);
            println!("now, it spent one minute!!: {}:{}", res_time.hour, res_time.minute);
            match time_view_colors.single_mut(){
                Ok((mut time_view_color, _)) => {
                    *time_view_color = BackgroundColor(Color::BLACK.with_alpha(res_time.get_brightness_from_time()));
                    println!("{}", res_time.get_brightness_from_time());
                }
                Err(e) => {println!("Error {e} is occured!!")}
            }
        }
    }
}

pub fn add_timer(
    mut commands: Commands,
){
    commands.spawn(TimeSpendTimer{
        timer: Timer::new(Duration::from_secs(ONE_MINUTE_SECOND as u64), TimerMode::Repeating)
    });
}

pub fn despawn_timer(
    mut comamnds: Commands,
    timer: Query<(Entity, &TimeSpendTimer)>,
){
    for (entity, _) in timer.iter(){
        comamnds.entity(entity).despawn();
    }
}

pub fn add_time_for_debug(
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