use bevy::{prelude::*, window::PrimaryWindow};

use super::components::Player;

pub const PLAYER_SPRITE_SIZE: f32 = 80.0;
pub fn spawn_player(
    mut commands :Commands,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    assests_server: Res<AssetServer>
){
    let window = windows_q.get_single().unwrap();
    commands.spawn((
        SpriteBundle{
            transform: Transform::from_xyz(window.width()/2.0, PLAYER_SPRITE_SIZE/2.0, 0.0),
            texture: assests_server.load("sprites/adventurer_back.png"),
            ..default()
        },
        Player{}
    ));
}