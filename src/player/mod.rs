use bevy::prelude::*;
use components::{HitPlayer, KillNpc};
use systems::*;
use upgrade_ui::interact_upgrade_button;

use crate::{spawn_score, systems::GameState};

pub mod systems;
pub mod components;
pub mod upgrade_ui;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<HitPlayer>()
        .add_event::<KillNpc>()
        .add_systems(Startup, (spawn_player_first_time, spawn_score).chain())
        .add_systems(Update, ((player_controller, hit_player, kill_npc, manage_xp).run_if(in_state(GameState::InGame)), interact_upgrade_button))
        ;
    }
}