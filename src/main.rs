use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_susie)
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



pub const PLAYER_SPRITE_SIZE: f32 = 80.0;
#[derive(Component)]
pub struct Susie {}
pub fn spawn_susie(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    assest_server: Res<AssetServer>
) {
    let window = window_q.get_single().unwrap();
    commands.spawn((
        SpriteBundle{
            transform: Transform::from_xyz(window.width()/2.0, PLAYER_SPRITE_SIZE/2.0, 0.0),
            texture: assest_server.load("sprites/adventurer_back.png"),
            ..default()
        },
        Susie{}
    ));
}
