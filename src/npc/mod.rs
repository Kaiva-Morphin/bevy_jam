use bevy::prelude::*;
use systems::*;

mod components;
pub mod systems;

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (spawn_civilian, spawn_hunter))
        .add_systems(Update, (manage_civilians, manage_hunters, manage_projectiles, process_proj_collisions))
        ;
    }
}