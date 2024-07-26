pub mod core;
pub mod player;
pub mod npc;
pub mod map;
pub mod systems;
pub mod characters;

use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;
use crate::characters::animation::CivilianAnims;

use bevy::prelude::*;

use characters::animation::{spawn_civilian_animation_bundle, spawn_player_animation_bundle, AnimationController};
use characters::plugin::CharacterAnimationPlugin;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
    ))
    .add_plugins((
        CharacterAnimationPlugin,
    ));
    app.add_systems(Update, update);
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
){
    let entity = spawn_civilian_animation_bundle(&mut commands, asset_server);
}



fn update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut controller: Query<&mut AnimationController>
){
    for mut controller in controller.iter_mut(){
        if keyboard.pressed(KeyCode::KeyA){controller.turn_left()};
        if keyboard.pressed(KeyCode::KeyD){controller.turn_right()};
        if keyboard.pressed(KeyCode::KeyS){controller.turn_down()};
        if keyboard.pressed(KeyCode::KeyW){controller.turn_up()};
        if keyboard.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD]){
            controller.play_walk();
        } else {
            controller.play_idle_priority(1);
        }
        if keyboard.just_pressed(KeyCode::Digit1){
            controller.arm();
        }
        if keyboard.just_pressed(KeyCode::Digit2){
            controller.disarm();
        }
        if keyboard.just_pressed(KeyCode::Digit3){
            controller.play_civil_attack();
        }
        if keyboard.just_pressed(KeyCode::Digit4){
            controller.play_hurt();
        }
    }
}