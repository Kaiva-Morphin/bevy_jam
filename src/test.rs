mod core;


use core::debug::{diagnostics_screen::plugin::ScreenDiagnosticsPlugin, egui_inspector::plugin::SwitchableEguiInspectorPlugin};

use bevy::prelude::*;
use bevy_rapier2d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default()
    ));
    app.add_systems(Update, update);
    app.add_systems(Startup, setup);
    app.run();
}

fn update(){
    
}

fn setup(){
    
}