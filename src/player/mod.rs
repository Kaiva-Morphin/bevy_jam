use bevy::prelude::*;
use components::{HitPlayer, KillNpc};
use systems::*;

use crate::systems::GameState;

pub mod systems;
pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<HitPlayer>()
        .add_event::<KillNpc>()
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_controller.run_if(in_state(GameState::InGame)))
        ;
    }
}