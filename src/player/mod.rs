mod components;
pub mod events;
mod systems;
use bevy::prelude::*;

use self::{
    events::AttackEvent,
    systems::{player_attack, player_attack_animation, spawn_player},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_attack)
            .add_systems(Update, player_attack_animation);
    }
}
