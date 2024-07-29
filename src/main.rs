pub mod core;
pub mod player;
pub mod npc;
pub mod map;
pub mod systems;
pub mod stuff;
pub mod characters;
pub mod sounds;

use crate::player::components::Player;
use core::{camera::plugin::EnhancedCameraPlugin, functions::TextureAtlasLayoutHandles};
use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use characters::plugin::CharacterAnimationPlugin;
use map::plugin::TileMapPlugin;
use npc::systems::RosesCollected;
use npc::NPCPlugin;
use player::PlayerPlugin;
use sounds::AudioPlugin;
use stuff::{simple_anim_update, spawn_follow_blood_particle, update_blood_particles};
use systems::*;

const NUM_ROSES: u32 = 6;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        //SwitchableEguiInspectorPlugin,
        //ScreenDiagnosticsPlugin,
        TileMapPlugin,
    ))
    .insert_state(GameState::InGame)
    .insert_resource(DayCycle {
        is_night: true,
        is_translating: false,
    })
    .insert_resource(RosesCollected {
        collected: 0,
        max: NUM_ROSES,
    })
    .add_event::<PauseEvent>()
    .add_plugins((
        PlayerPlugin,
        NPCPlugin,
        CharacterAnimationPlugin,
        AudioPlugin,
    ))
    .add_systems(Startup, spawn_starter_screen)
    .add_systems(Update, interact_start_button)
    .add_systems(Update, (
        (update_daycycle, update_score).run_if(in_state(GameState::InGame)), 
        simple_anim_update.run_if(in_state(GameState::InGame)),
        update_blood_particles.run_if(in_state(GameState::InGame)),
        pause_game
    ))
    .run();
}
