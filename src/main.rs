mod core;
mod player;
mod npc;
pub mod systems;

use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
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
        RapierDebugRenderPlugin::default(),
        RapierPhysicsPlugin::<NoUserData>::default(),
    ))
    .insert_resource(DayCycle {time: 0., is_day: true})
    .add_plugins((
        PlayerPlugin,
        NPCPlugin,
    ))
    .add_systems(Update, update_daycycle)
    .run();
}


fn update(){
    
}