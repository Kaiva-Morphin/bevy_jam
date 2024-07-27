use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::characters::animation::{AnimationController, PartType};
use crate::core::camera::plugin::CameraFollow;
use crate::systems::DayCycle;
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
pub const LAT_CG: u32 = 0b0000_0000_0001_0000;

#[derive(Component)]
pub struct PlayerController{
    pub accumulated_velocity: Vec2,
}
impl Default for PlayerController {
    fn default() -> Self {
        PlayerController{accumulated_velocity: Vec2::ZERO}
    }
}

#[derive(Default, Debug)]
pub enum Direction {
    Up,
    Right,
    #[default]
    Down,
    Left
}

#[derive(Component, Default)]
pub struct PlayerAnimationState{
    pub dir: Direction
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
){
    commands.spawn((
        VisibilityBundle::default(),
        TransformBundle::from_transform(Transform::from_xyz(16., 16., 0.)),
        Name::new("Player"),
        CameraFollow{order: 0, speed: 10.},
        KinematicCharacterController::default(),
        PlayerController::default(),
        Player {hp: 100, xp: 0, score: 0, max_speed: 80., accumulation_grain: 600.},
        AnimationController::default(),
        RigidBody::KinematicPositionBased,
        Collider::ball(4.),
        // ActiveCollisionTypes::all(),
        ActiveEvents::COLLISION_EVENTS,
        // CollisionGroups::new(
        //     Group::from_bits(PLAYER_CG).unwrap(),
        //     Group::from_bits(BULLET_CG | STRUCTURES_CG | NPC_CG | LAT_CG).unwrap()
        // ),
        (SolverGroups::new(
            Group::NONE,
            Group::NONE
        ),
        CollisionGroups::new(
            Group::NONE,
            Group::NONE
        ),
        Sensor,
        ),
        DashTimer {timer: Timer::new(Duration::from_secs_f32(0.35), TimerMode::Repeating)},
        Sleeping::disabled()
    )).with_children(|commands| {commands.spawn((
        PartType::Body{variant: 0, variants: 1},
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

pub fn player_controller(
    mut commands: Commands,
    mut player_q: Query<(&mut KinematicCharacterController, &mut PlayerController,
        &mut AnimationController, &mut DashTimer, &mut Player, Entity, &CollisionGroups)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    day_cycle: Res<DayCycle>,
    time: Res<Time>,
    mut dash_dir: Local<Vec2>,
) {
    let (mut character_controller, mut controller,
        mut animation_controller, mut dash_timer,
        mut player, player_entity, gr) = player_q.single_mut();
    let dt = time.delta_seconds();
    // println!("{:?}", gr);
    if dash_timer.timer.elapsed_secs() == 0. {
        let input_dir = vec2(
            keyboard.pressed(KeyCode::KeyD) as i32 as f32 - keyboard.pressed(KeyCode::KeyA) as i32 as f32,
            keyboard.pressed(KeyCode::KeyW) as i32 as f32 - keyboard.pressed(KeyCode::KeyS) as i32 as f32
        );
        
        controller.accumulated_velocity = controller.accumulated_velocity.move_towards(input_dir.normalize_or_zero() * player.max_speed, dt * player.accumulation_grain);
        if controller.accumulated_velocity.length() > player.max_speed {controller.accumulated_velocity = controller.accumulated_velocity.normalize() * player.max_speed}
        character_controller.translation = Some(controller.accumulated_velocity * dt);
    
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
    
        if keyboard.just_released(KeyCode::ShiftLeft) {
            dash_timer.timer.tick(Duration::from_secs_f32(dt));
            *dash_dir = input_dir;
            if day_cycle.is_night {
                commands.entity(player_entity).insert(
                    CollisionGroups::new(
                        Group::from_bits(PLAYER_CG).unwrap(),
                        Group::from_bits(STRUCTURES_CG | LAT_CG).unwrap()
                    ),
                );
            } else {
                // commands.entity(player_entity).insert(
                //     CollisionGroups::new(
                //         Group::from_bits(PLAYER_CG).unwrap(),
                //         Group::from_bits(STRUCTURES_CG).unwrap()
                //     ),
                // );
                println!("я ебал рапиру в рот {:?}", CollisionGroups::new(
                    Group::NONE,
                    Group::NONE
                ));
                commands.entity(player_entity).insert(
                    CollisionGroups::new(
                        Group::NONE,
                        Group::NONE
                    ),
                );
            }
        }
    } else {
        dash_timer.timer.tick(Duration::from_secs_f32(dt));
        let t = dash_timer.timer.elapsed_secs();

        let new_max = player.max_speed * g(t);
        let new_gain = player.accumulation_grain * g(t);

        controller.accumulated_velocity = controller.accumulated_velocity.move_towards(dash_dir.normalize_or_zero() * new_max, dt * new_gain);
        if controller.accumulated_velocity.length() > new_max {controller.accumulated_velocity = controller.accumulated_velocity.normalize() * new_max}
        character_controller.translation = Some(controller.accumulated_velocity * dt);
        
        if dash_timer.timer.finished() {
            dash_timer.timer.set_elapsed(Duration::from_secs_f32(0.));
            commands.entity(player_entity).insert(
            CollisionGroups::new(
                Group::from_bits(PLAYER_CG).unwrap(),
                Group::from_bits(BULLET_CG | STRUCTURES_CG | NPC_CG | LAT_CG).unwrap()
            ));
        }
    }
}

pub fn player_stat(
    mut player: Query<&mut Player>,
) {
    let player = player.single_mut();
    if player.hp < 1 {
        // dead todo:
    }
}

fn g(x: f32) -> f32 {
    let x = 3. - 5. * x;
    5. * std::f32::consts::E.powf(-(x - 1.639964).powf(2.)/(2.*0.800886f32.powf(2.)))
}