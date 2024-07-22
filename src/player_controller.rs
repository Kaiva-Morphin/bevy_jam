mod core;
mod map;


use core::camera::plugin::CameraFollow;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::{ScreenDiagnostics, ScreenDiagnosticsPlugin};

use bevy::input::keyboard::KeyboardInput;
use bevy::math::vec2;
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use map::plugin::TileMapPlugin;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        TileMapPlugin
    ));
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
    app.run();
}

#[derive(Component)]
pub struct PlayerController{
    accumulated_velocity: Vec2,
}
impl Default for PlayerController {
    fn default() -> Self {
        PlayerController{accumulated_velocity: Vec2::ZERO}
    }
}

fn setup(
    mut commands: Commands,
){
    commands
        .spawn(TransformBundle::default())
        .insert(CameraFollow{order: 0, speed: 10_000.})
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(5.))
        .insert(KinematicCharacterController::default())
        .insert(PlayerController::default());
    
}

struct SpeedCFG{
    max_speed : f32,
    accumulation_grain : f32,
    follow_speed: f32
}

impl Default for SpeedCFG {
    fn default() -> Self {
        SpeedCFG {
            max_speed: 80.,
            accumulation_grain: 300.,
            follow_speed: 10_000.
        }
    }
}

fn update(
    mut player_q: Query<(&mut KinematicCharacterController, &mut PlayerController, &mut CameraFollow)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dbg: ResMut<ScreenDiagnostics>,
    time: Res<Time>,
    mut speed_cfg: Local<SpeedCFG>,
    mut egui_context: EguiContexts,
){  
    let ctx = egui_context.ctx_mut();

    egui::Window::new("SLIDERS").show(ctx, |ui|{
        ui.add(Slider::new(&mut speed_cfg.max_speed, 1. ..= 10_000.).text("MAX SPEED"));
        ui.add(Slider::new(&mut speed_cfg.accumulation_grain, 1. ..= 10_000.).text("ACCUMULATION GRAIN"));
        ui.add(Slider::new(&mut speed_cfg.follow_speed, 1. ..= 10_000.).text("CAMERA FOLLOW SPEED"));
    });

    let (mut p, mut controller, mut follow) = player_q.single_mut();
    
    follow.speed = speed_cfg.follow_speed;

    let input_dir = vec2(
        keyboard.pressed(KeyCode::KeyD) as i32 as f32 - keyboard.pressed(KeyCode::KeyA) as i32 as f32,
        keyboard.pressed(KeyCode::KeyW) as i32 as f32 - keyboard.pressed(KeyCode::KeyS) as i32 as f32
    );
    controller.accumulated_velocity = controller.accumulated_velocity.move_towards(input_dir.normalize_or_zero() * speed_cfg.max_speed, time.delta_seconds() * speed_cfg.accumulation_grain);
    if controller.accumulated_velocity.length() > speed_cfg.max_speed {controller.accumulated_velocity = controller.accumulated_velocity.normalize() * speed_cfg.max_speed}
    p.translation = Some(controller.accumulated_velocity * time.delta_seconds());


}
