mod player;

use bevy::{prelude::*, window::PrimaryWindow};
use player::PlayerPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_startup_system(spawn_camera)
    .run();
}

pub fn spawn_camera(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.get_single().unwrap();
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        ..default()
    });
}
