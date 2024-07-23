use bevy::prelude::*;
use components::NpcsOnMap;
use systems::*;
use pathfinder::*;

mod components;
mod pathfinder;
pub mod systems;

pub struct NPCPlugin;

impl Plugin for NPCPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(NpcsOnMap::default())
        // .add_systems(Startup, (spawn_civilian, spawn_hunter))
        .add_systems(Update, (manage_civilians, manage_hunters, manage_projectiles,
            process_proj_collisions, entity_spawner))
        ;
    }
}