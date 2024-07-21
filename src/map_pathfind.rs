mod core;
mod map;


use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;

use map::plugin::TileMapPlugin;


use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        
    ));
    app.add_plugins(
        TileMapPlugin
    );
    app.run();
}

