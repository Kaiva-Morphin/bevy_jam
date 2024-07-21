use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};
use bevy_rapier2d::prelude::*;

use crate::{player::components::Player, BULLET_CG, NPC_CG, PLAYER_CG, STRUCTURES_CG};

use super::components::*;

const THRESHOLD: f32 = 100.0;
const UPP_THRESHOLD: f32 = THRESHOLD * 2.0;
const CIV_MS: f32 = 30.0;
const HUN_MS: f32 = 40.0;
const PROJ_V: f32 = 150.0;
const HUNTER_TIMER: f32 = 1.0;

pub fn spawn_civilian(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/box.png"),
            transform: Transform::from_xyz(40., 40., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(10., 10.),
        GravityScale(0.),
        Civilian,
        LockedAxes::ROTATION_LOCKED_Z,
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        },
        Damping {
            linear_damping: 1.2,
            angular_damping: 0.0,
        },
        SolverGroups::new(
            Group::from_bits(NPC_CG).unwrap(),
            Group::from_bits(PLAYER_CG & STRUCTURES_CG).unwrap()
        ),
    ));
}

pub fn manage_civilians(
    mut civilians_data: Query<(&Transform, &mut Velocity), With<Civilian>>,
    player_transform: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_pos = player_transform.single().translation;
    let dt = time.delta_seconds();
    for (civ_transform, mut civ_velocity) in civilians_data.iter_mut() {
        let civ_pos = civ_transform.translation;
        let civ_vel = &mut civ_velocity.linvel;
        let direction = (civ_pos - player_pos).xy();
        let length = direction.length();
        if length < THRESHOLD {
            *civ_vel += (direction / length) * CIV_MS * dt;
        }
    }
}

pub fn spawn_hunter(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/4el.png"),
            transform: Transform::from_xyz(-40., -40., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(10., 10.),
        GravityScale(0.),
        Hunter,
        LockedAxes::ROTATION_LOCKED_Z,
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        },
        Damping {
            linear_damping: 1.2,
            angular_damping: 0.0,
        },
        SolverGroups::new(
            Group::from_bits(NPC_CG).unwrap(),
            Group::from_bits(PLAYER_CG & STRUCTURES_CG).unwrap()
        ),
        HunterTimer { timer: Timer::new(Duration::from_secs_f32(HUNTER_TIMER), TimerMode::Repeating) }
    ));
}

pub fn manage_hunters(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut hunters_data: Query<(&Transform, &mut Velocity, &mut HunterTimer), Without<Player>>,
    player_data: Query<(&Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    let player_data = player_data.single();
    let player_pos = player_data.0.translation.xy();
    let player_vel = player_data.1.linvel;
    let dt = time.delta_seconds();
    for (hunter_transform, mut hunter_velocity, mut hunter_timer) in hunters_data.iter_mut() {
        hunter_timer.timer.tick(Duration::from_secs_f32(dt));
        let hunter_pos = hunter_transform.translation.xy();
        let direction = player_pos - hunter_pos;
        let length = direction.length();
        if length < THRESHOLD {
            hunter_velocity.linvel -= (direction / length) * HUN_MS * dt
        } else if length < UPP_THRESHOLD {
            hunter_velocity.linvel += (direction / length) * HUN_MS * dt
        }
        if hunter_timer.timer.finished() {
            if let Some(intercept) = calculate_intercept(hunter_pos, player_pos, player_vel, PROJ_V) {
                let dir = intercept - hunter_pos;
                let dir = dir / dir.length();
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("sprites/box.png"), // todo: rm later
                        transform: Transform::from_translation(hunter_pos.extend(0.)),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(10., 10.),
                    SolverGroups::new(
                        Group::from_bits(BULLET_CG).unwrap(),
                        Group::from_bits(PLAYER_CG).unwrap()
                    ),
                    GravityScale(0.),
                    LockedAxes::ROTATION_LOCKED_Z,
                    Velocity {
                        linvel: PROJ_V * dir,
                        angvel: 0.0,
                    },
                    DespawnTimer { timer: Timer::new(Duration::from_secs(6), TimerMode::Once) },
                    Projectile,
                ));
            }
        }
    }
}

fn calculate_intercept(shooter_pos: Vec2, target_pos: Vec2, target_vel: Vec2, proj_vel: f32) -> Option<Vec2> {
    let direction = target_pos - shooter_pos;
    let a = target_vel.dot(target_vel) - proj_vel * proj_vel;
    let b = 2. * direction.dot(target_vel);
    let c = direction.dot(direction);
    let dis = b * b - 4. * a * c;
    if dis < 0. {
        return None;
    }
    let t = (-b - dis.sqrt()) / (2. * a);
    if t < 0. {
        return None;
    }
    let i = target_pos + target_vel * t;
    return Some(i);
}

pub fn manage_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(&mut DespawnTimer, Entity), With<Projectile>>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (mut timer, entity) in projectiles.iter_mut() {
        timer.timer.tick(Duration::from_secs_f32(delta));
        if timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn process_proj_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<&mut Player>,
    projectiles: Query<&Projectile>
) {
    let player = player.single_mut();
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(_, proj_entity, _) = collision_event {
            if let Ok(_) = projectiles.get(*proj_entity) {
                commands.entity(*proj_entity).despawn();
            }
        }
    }
}