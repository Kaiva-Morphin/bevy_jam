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
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;
use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use characters::plugin::CharacterAnimationPlugin;
use map::plugin::TileMapPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;
use sounds::AudioPlugin;
use stuff::{simple_anim_update, spawn_follow_blood_particle, update_blood_particles};
use systems::*;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        TileMapPlugin,
    ))
    .insert_state(GameState::InGame)
    .insert_resource(DayCycle {
        is_night: true,
        is_translating: false,
    })
    .add_plugins((
        PlayerPlugin,
        NPCPlugin,
        CharacterAnimationPlugin,
        AudioPlugin,
    ))
    .add_systems(Update, (
        update_daycycle.run_if(in_state(GameState::InGame)), 
        simple_anim_update.run_if(in_state(GameState::InGame)),
        update_blood_particles.run_if(in_state(GameState::InGame)),
        pause_game
    ))
    .add_systems(Update, test)
    .run();
}

use rand::Rng;

pub fn test(
    mut commands: Commands,
    mut layout_handles: ResMut<TextureAtlasLayoutHandles>,
    asset_server: Res<AssetServer>,
    follow: Query<Entity, With<Player>>,
    mut spawned: Local<bool>
){
    if *spawned{return}
    if follow.is_empty(){return}
    let follow = follow.single();
    let pos = vec3(
        rand::thread_rng().gen::<f32>() * 100.,
        rand::thread_rng().gen::<f32>() * 100.,
        10.
    );
    spawn_follow_blood_particle(&mut commands, &mut layout_handles, &asset_server, follow, pos, 100);
    *spawned = true
}