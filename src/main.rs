mod core;
mod characters;
use characters::plugin::{self, CharacterGeneratorViewerPlugin};
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        CharacterGeneratorViewerPlugin,
    ));
    app.run();
}


fn update(){
    
}