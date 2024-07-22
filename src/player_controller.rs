mod core;
mod map;


use core::camera::plugin::CameraFollow;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::{ScreenDiagnostics, ScreenDiagnosticsPlugin};
use core::default;

use bevy::input::keyboard::KeyboardInput;
use bevy::math::{uvec2, vec2};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use map::plugin::TileMapPlugin;
use pathfinding::num_traits::Signed;

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

#[derive(Default, Debug)]
enum Direction {
    Up,
    Right,
    #[default]
    Down,
    Left
}

#[derive(Component, Default)]
struct PlayerAnimationState{
    pub dir: Direction
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
){
    commands.spawn((
        VisibilityBundle::default(),
        TransformBundle::default(),
        Name::new("Player"),
        CameraFollow{order: 0, speed: 10_000.},
        RigidBody::KinematicPositionBased,
        Collider::ball(5.),
        KinematicCharacterController::default(),
        PlayerController::default(),
    )).with_children(|commands| {commands.spawn((
        SpriteBundle{
            texture: asset_server.load("vampire.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(14, 18), 3, 4, None, None)),
            index: 2
        },
        PlayerAnimationState::default()
    ));});
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
            accumulation_grain: 600.,
            follow_speed: 10.
        }
    }
}


fn update(
    mut player_q: Query<(&mut KinematicCharacterController, &mut PlayerController, &mut CameraFollow), Without<PlayerAnimationState>>,
    mut player_sprite_q: Query<(&mut TextureAtlas, &mut Transform, &mut PlayerAnimationState), Without<PlayerController>>,
    keyboard: Res<ButtonInput<KeyCode>>,
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

    let (mut layout, mut transform, mut anim) = player_sprite_q.single_mut();
    
    if input_dir.x.abs() < 0.1 { // x axis is priotirized 
        if input_dir.y.abs() > 0.1 {
            if input_dir.y.is_positive(){anim.dir = Direction::Up}
            if input_dir.y.is_negative(){anim.dir = Direction::Down}
        }
    } else {
        if input_dir.x.is_positive(){anim.dir = Direction::Right}
        if input_dir.x.is_negative(){anim.dir = Direction::Left}
    }

    let index = match anim.dir {
        Direction::Up => {6},
        Direction::Right => {9},
        Direction::Down => {0},
        Direction::Left => {3},
    };

    transform.translation.y = ((time.elapsed_seconds() * 5.) as i32 % 2) as f32 * 0.5;
    if controller.accumulated_velocity.length() > 0.1 {
        let mut anim_offset = ((time.elapsed_seconds() * 5.) as i32 % 4) as usize;
        if anim_offset == 3 {anim_offset = 1} // wrap
        layout.index = index + anim_offset;
    } else {
        layout.index = index + 1;
    }
}
