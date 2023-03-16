mod systems;
mod components;
use bevy::prelude::*;

use self::systems::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}