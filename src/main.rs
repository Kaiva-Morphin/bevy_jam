pub mod core;
pub mod player;
pub mod npc;
pub mod map;
pub mod systems;
pub mod characters;
use crate::player::components::Player;
use core::camera::plugin::EnhancedCameraPlugin;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;

use bevy::prelude::*;

use characters::plugin::CharacterAnimationPlugin;
use map::plugin::TileMapPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;
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
    .insert_resource(DayCycle {time: 0., is_day: true})
    .add_plugins((
        PlayerPlugin,
        NPCPlugin,
        CharacterAnimationPlugin,
    ))
    .add_systems(Update, update_daycycle)
    .run();
}