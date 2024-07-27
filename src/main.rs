pub mod core;
pub mod player;
pub mod npc;
pub mod map;
pub mod systems;
pub mod stuff;
pub mod characters;
pub mod sounds;

use crate::player::components::Player;
use core::camera::plugin::EnhancedCameraPlugin;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;
use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use characters::plugin::CharacterAnimationPlugin;
use map::plugin::TileMapPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;
use sounds::AudioPlugin;
use stuff::simple_anim_update;
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
        cycle_timer: Timer::new(Duration::from_secs_f32(DAY_DURATION), TimerMode::Repeating),
        translation_timer: Timer::new(Duration::from_secs_f32(TRANSLATION_DURATION), TimerMode::Repeating),
        daytime: 0.,
        is_night: true,
        is_translating: false,
    })
    .add_plugins((
        PlayerPlugin,
        NPCPlugin,
        CharacterAnimationPlugin,
        AudioPlugin,
    ))
    .add_systems(Update, (update_daycycle.run_if(in_state(GameState::InGame)), pause_game))
    .add_systems(Update, simple_anim_update.run_if(in_state(GameState::InGame)))
    .run();
}