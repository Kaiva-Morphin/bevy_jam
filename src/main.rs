mod core;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin
    ));
    app.run();
}


fn update(){
    
}