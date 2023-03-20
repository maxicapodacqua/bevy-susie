use bevy::prelude::*;

use self::systems::{move_enemy, spawn_enemy};
mod components;
mod systems;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy).add_system(move_enemy);
    }
}
