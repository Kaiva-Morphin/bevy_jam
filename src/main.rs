mod core;
mod player;
mod npc;
pub mod systems;

use core::camera::plugin::EnhancedCameraPlugin;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;

use bevy::prelude::*;

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
    ))
    .insert_resource(DayCycle {time: 0., is_day: true})
    .add_plugins((
        PlayerPlugin,
        NPCPlugin,
    ))
    .add_systems(Update, update_daycycle)
    .run();
}