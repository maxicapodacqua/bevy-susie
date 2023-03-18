use bevy::{prelude::*, window::PrimaryWindow, transform};
use rand::random;
// use rand::random;

use super::components::Enemy;

pub fn spawn_enemy(
    mut commands :Commands,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    assests_server: Res<AssetServer>
){


   let win = windows_q.get_single().unwrap();

    // let direction = if random::<bool>() {1.0} else {-1.0};
    let direction = 1.00;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(60.0, win.height() / 2.0, 0.0),
            texture: assests_server.load("sprites/zombie_walk1.png"),
            ..default()
        },
        Enemy {direction: direction}
    ));
}


pub fn move_enemy(
    mut enemy_q: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
){

    for (mut tranform, enemy) in enemy_q.iter_mut() {
        let x_trans = enemy.direction * 28.0 * time.delta_seconds();
        tranform.translation += Vec3::new(x_trans,0.0  , 0.0) 
    }
}