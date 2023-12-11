use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;
// use rand::random;

use super::components::Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    assests_server: Res<AssetServer>,
) {
    let win = windows_q.get_single().unwrap();

    let (spawn_location, direction) = get_random_location_direction(win);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(spawn_location, win.height() / 2.0, 0.0),
            texture: assests_server.load("sprites/zombie_walk1.png"),
            ..default()
        },
        Enemy {
            direction: direction,
        },
    ));
}

fn get_random_location_direction(win: &Window) -> (f32, f32) {
    return if random::<bool>() {
        (win.width(), -1.0)
    } else {
        (0.0, 1.0)
    };
}

pub fn move_enemy(mut enemy_q: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut tranform, enemy) in enemy_q.iter_mut() {
        let x_trans = enemy.direction * 58.0 * time.delta_seconds();
        tranform.translation += Vec3::new(x_trans, 0.0, 0.0)
    }
}
