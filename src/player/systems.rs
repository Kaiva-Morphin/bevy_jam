use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::characters::animation::{spawn_player_animation_bundle, AnimationController, PartType};
use crate::core::camera::plugin::CameraFollow;
use crate::core::functions::TextureAtlasLayoutHandles;
use crate::core::ui::PlayerUINode;
use crate::sounds::components::PlaySoundEvent;
use crate::systems::DayCycle;
use crate::PauseEvent;
use bevy::math::{uvec2, vec2};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, Slider};
use pathfinding::num_traits::Signed;

use super::components::*;
use super::upgrade_ui::lvl_up;

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
    asset_server: Res<AssetServer>,
    mut layout_handles: ResMut<TextureAtlasLayoutHandles>,
){
    let entity = spawn_player_animation_bundle(&mut commands, &asset_server, &mut layout_handles);
    commands.entity(entity).insert((
        VisibilityBundle::default(),
        TransformBundle::from_transform(Transform::from_xyz(16., 16., 0.)),
        Name::new("Player"),
        CameraFollow{order: 0, speed: 10.},
        Player {hp: 100., xp: 0., score: 0., max_speed: 80., accumulation_grain: 600., 
            phys_res: 0.2, hp_gain: 10., xp_gain: 10., max_xp: 100., max_hp: 100. },
        AnimationController::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::ball(4.),
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        PlayerController::default(),
        DashTimer {timer: Timer::new(Duration::from_secs_f32(0.35), TimerMode::Repeating)},
        Sleeping::disabled(),
        CollisionGroups::new(
            Group::from_bits(PLAYER_CG).unwrap(),
            Group::from_bits(BULLET_CG | STRUCTURES_CG | NPC_CG).unwrap()
        ),
    ));
    // .with_children(|commands| {commands.spawn((
    //     PartType::Body{variant: 0, variants: 1},
    //     SpriteBundle{
    //         texture: asset_server.load("player/vampire.png"),
    //         ..default()
    //     },
    //     TextureAtlas{
    //         layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(14, 20), 7, 3, Some(uvec2(1, 1)), None)),
    //         index: 2
    //     },
    // ));});
}

pub fn player_controller(
    mut commands: Commands,
    mut player_q: Query<(&mut Velocity, &mut PlayerController,
        &mut AnimationController, &mut DashTimer, &mut Player, Entity)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    day_cycle: Res<DayCycle>,
    time: Res<Time>,
    mut dash_dir: Local<Vec2>,
    mut play_sound: EventWriter<PlaySoundEvent>,
) {
    let (mut character_controller, mut controller,
        mut animation_controller, mut dash_timer,
        mut player, player_entity) = player_q.single_mut();
    character_controller.linvel = Vec2::ZERO;
    let dt = time.delta_seconds();
    if dash_timer.timer.elapsed_secs() == 0. {
        let input_dir = vec2(
            keyboard.pressed(KeyCode::KeyD) as i32 as f32 - keyboard.pressed(KeyCode::KeyA) as i32 as f32,
            keyboard.pressed(KeyCode::KeyW) as i32 as f32 - keyboard.pressed(KeyCode::KeyS) as i32 as f32
        );
        
        controller.accumulated_velocity = controller.accumulated_velocity.move_towards(input_dir.normalize_or_zero() * player.max_speed, dt * player.accumulation_grain);
        if controller.accumulated_velocity.length() > player.max_speed {controller.accumulated_velocity = controller.accumulated_velocity.normalize() * player.max_speed}
        character_controller.linvel = controller.accumulated_velocity;
    
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
            play_sound.send(PlaySoundEvent::Dash);
            dash_timer.timer.tick(Duration::from_secs_f32(dt));
            *dash_dir = input_dir;
            if day_cycle.is_night {
                commands.entity(player_entity).insert(
                    (CollisionGroups::new(
                        Group::from_bits(PLAYER_CG).unwrap(),
                        Group::from_bits(STRUCTURES_CG | NPC_CG).unwrap()
                    ),
                    Sensor,)
                );
            } else {
                commands.entity(player_entity).insert(
                    CollisionGroups::new(
                        Group::from_bits(PLAYER_CG).unwrap(),
                        Group::from_bits(STRUCTURES_CG).unwrap()
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
        character_controller.linvel = controller.accumulated_velocity;
        
        if dash_timer.timer.finished() {
            dash_timer.timer.set_elapsed(Duration::from_secs_f32(0.));
            commands.entity(player_entity).insert(
            CollisionGroups::new(
                Group::from_bits(PLAYER_CG).unwrap(),
                Group::from_bits(BULLET_CG | STRUCTURES_CG | NPC_CG).unwrap()
            )).remove::<Sensor>();
        }
    }
}

fn g(x: f32) -> f32 {
    let x = 3. - 5. * x;
    5. * std::f32::consts::E.powf(-(x - 1.639964).powf(2.)/(2.*0.800886f32.powf(2.)))
}

pub fn hit_player(
    mut commands: Commands,
    mut hit_player: EventReader<HitPlayer>,
    mut player: Query<(&mut Player, &mut AnimationController, Entity)>,
    mut play_sound: EventWriter<PlaySoundEvent>,
) {
    let (mut player, mut animation_controller, player_entity) = player.single_mut();
    for hit in hit_player.read() {
        println!("{:?}", player.hp);
        animation_controller.play_hurt();
        if hit.dmg_type == 0 { // proj
            player.hp -= player.max_hp * 0.1 * (1. - player.phys_res)
        } else if hit.dmg_type == 1 { // civ
            player.hp -= player.max_hp * 0.05 * (1. - player.phys_res)
        } else if hit.dmg_type == 2 { // hun
            player.hp -= 15. * (1. - player.phys_res)
        }
    }
    if player.hp < 0. {
        play_sound.send(PlaySoundEvent::Kill);
        commands.entity(player_entity).despawn_recursive();
    }
}

pub fn kill_npc(
    mut kill_npc: EventReader<KillNpc>,
    mut player: Query<&mut Player>,
) {
    let mut player = player.single_mut();
    for kill in kill_npc.read() {
        player.hp = (player.hp + player.hp_gain).clamp(0.0, player.max_hp);
        if kill.npc_type == 0 { // civ
            player.score += 100.;
            player.xp += player.xp_gain;
        } else if kill.npc_type == 1 { // hun
            player.score += 500.;
            player.xp += player.xp_gain * 3.;
        }
    }
}

pub fn manage_xp(
    mut player: Query<&mut Player>,
    mut play_sound: EventWriter<PlaySoundEvent>,
    mut commands: Commands,
    mut pause_event: EventWriter<PauseEvent>,
    asset_server: Res<AssetServer>,
    mut t: Local<bool>,
) {
    let mut player = player.single_mut();
    if player.xp > player.max_xp {
        player.xp -= player.max_hp;
        player.max_xp *= 1.2;
        play_sound.send(PlaySoundEvent::LvlUp);
        lvl_up(&mut commands, &asset_server);
        pause_event.send(PauseEvent);
        *t = true;
    }
}