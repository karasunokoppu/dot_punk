use crate::core::resource::ActiveDatas;
use crate::core::resource::InWorldTime;
use crate::game::world::player;
use crate::game::world::stage;
use crate::{GameState, core::systems::despawn_screen};
use bevy::prelude::*;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash_setup)
        .insert_resource(InWorldTime::default())
        .insert_resource(ActiveDatas::default())
        .add_plugins((stage::stage_plugin, player::player_plugin))
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("splash/icon.png");
    // Display the logo
    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnSplashScreen,
        children![(
            ImageNode::new(icon),
            Node {
                // This will set the logo to be 200px wide, and auto adjust its height
                width: Val::Px(200.0),
                ..default()
            },
        )],
    ));
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu);
    }
}
