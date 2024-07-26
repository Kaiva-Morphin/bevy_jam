use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::characters::animation::AnimationController;
use crate::core::camera::plugin::CameraFollow;
use bevy::math::{uvec2, vec2};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::control::KinematicCharacterController;
use pathfinding::num_traits::Signed;

use super::components::*;

pub const PLAYER_CG: u32 = 0b0000_0000_0000_0001;
pub const NPC_CG: u32 = 0b0000_0000_0000_0010;
pub const STRUCTURES_CG: u32 = 0b0000_0000_0000_0100;
pub const BULLET_CG: u32 = 0b0000_0000_0000_1000;

#[derive(Component)]
pub struct PlayerController{
    pub accumulated_velocity: Vec2,
}
impl Default for PlayerController {
    fn default() -> Self {
        PlayerController{accumulated_velocity: Vec2::ZERO}
    }
}

pub fn spawn_player(
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
        Player,
        ActiveEvents::COLLISION_EVENTS,
        AnimationController::default(),
        CollisionGroups::new(
            Group::from_bits(PLAYER_CG).unwrap(),
            Group::from_bits(BULLET_CG | STRUCTURES_CG).unwrap()
        ),
    )).with_children(|commands| {commands.spawn((
        SpriteBundle{
            texture: asset_server.load("player/vampire.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(14, 20), 7, 3, Some(uvec2(1, 1)), None)),
            index: 2
        },
    ));});
}



pub struct SpeedCFG{
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

pub fn player_controller(
    mut player_q: Query<(&mut KinematicCharacterController, &mut PlayerController, &mut CameraFollow, &Transform), With<Player>>,
    mut animation_controller: Query<&mut AnimationController, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut speed_cfg: Local<SpeedCFG>,
    mut egui_context: EguiContexts,
){  
    
    //
    let ctx = egui_context.ctx_mut();
    egui::Window::new("SLIDERS").show(ctx, |ui|{
        ui.add(Slider::new(&mut speed_cfg.max_speed, 1. ..= 10_000.).text("MAX SPEED"));
        ui.add(Slider::new(&mut speed_cfg.accumulation_grain, 1. ..= 10_000.).text("ACCUMULATION GRAIN"));
        ui.add(Slider::new(&mut speed_cfg.follow_speed, 1. ..= 10_000.).text("CAMERA FOLLOW SPEED"));
    });

    let (mut character_controller, mut controller, mut follow, player_transform) = player_q.single_mut();
    let mut animation_controller = animation_controller.single_mut();


    follow.speed = speed_cfg.follow_speed;

    let input_dir = vec2(
        keyboard.pressed(KeyCode::KeyD) as i32 as f32 - keyboard.pressed(KeyCode::KeyA) as i32 as f32,
        keyboard.pressed(KeyCode::KeyW) as i32 as f32 - keyboard.pressed(KeyCode::KeyS) as i32 as f32
    );

    controller.accumulated_velocity = controller.accumulated_velocity.move_towards(input_dir.normalize_or_zero() * speed_cfg.max_speed, time.delta_seconds() * speed_cfg.accumulation_grain);
    if controller.accumulated_velocity.length() > speed_cfg.max_speed {controller.accumulated_velocity = controller.accumulated_velocity.normalize() * speed_cfg.max_speed}
    character_controller.translation = Some(controller.accumulated_velocity * time.delta_seconds());
    
    

    //let (mut layout, mut transform, mut anim) = player_sprite_q.single_mut();
    
    if input_dir.x.abs() < 0.1 { // x axis is priotirized 
        if input_dir.y.abs() > 0.1 {
            if input_dir.y.is_positive(){animation_controller.turn_up()}
            if input_dir.y.is_negative(){animation_controller.turn_down()}
        }
    } else {
        if input_dir.x.is_positive(){animation_controller.turn_right()}
        if input_dir.x.is_negative(){animation_controller.turn_left()}
    }
    if controller.accumulated_velocity.length() > 0.1 {
        animation_controller.play_walk();
    } else {
        animation_controller.play_idle_priority(1);
    }
    if keyboard.just_pressed(KeyCode::KeyB){
        animation_controller.play_hurt();
    }
    /*transform.translation.y = ((time.elapsed_seconds() * 5.) as i32 % 2) as f32 * 0.5;
    if controller.accumulated_velocity.length() > 0.1 {
        let mut anim_offset = ((time.elapsed_seconds() * 5.) as i32 % 4) as usize;
        if anim_offset == 3 {anim_offset = 1} // wrap
        layout.index = index + anim_offset;
    } else {
        layout.index = index + 1;
    }*/
}
