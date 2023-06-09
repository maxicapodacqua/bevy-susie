use bevy::{prelude::*, window::PrimaryWindow};

use super::{components::Player, events::AttackEvent};

pub const PLAYER_SPRITE_SIZE: f32 = 80.0;
pub fn spawn_player(
    mut commands: Commands,
    windows_q: Query<&Window, With<PrimaryWindow>>,
    assests_server: Res<AssetServer>,
) {
    let window = windows_q.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, PLAYER_SPRITE_SIZE / 2.0, 0.0),
            texture: assests_server.load("sprites/adventurer_back.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_attack(key_input: Res<Input<KeyCode>>, mut attack_evt: EventWriter<AttackEvent>) {
    if key_input.pressed(KeyCode::Space) {
        attack_evt.send_default();
    }
}

pub fn player_attack_animation(
    mut player_q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut attack_evt: EventReader<AttackEvent>,
) {
    if !attack_evt.is_empty() {
        attack_evt.clear();
        if let Ok(mut trans) = player_q.get_single_mut() {
            trans.translation.y += 200. * time.delta_seconds();
        }
    }
}
