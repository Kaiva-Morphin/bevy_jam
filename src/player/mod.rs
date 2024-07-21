use bevy::prelude::*;
use systems::*;

mod systems;
pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_controller)
        ;
    }
}