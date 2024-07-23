use std::time::Duration;

use bevy::{color::palettes::css::RED, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{map::{plugin::TrespassableCells, tilemap::TransformToGrid}, player::{components::Player, systems::PlayerController}, BULLET_CG, NPC_CG, PLAYER_CG, STRUCTURES_CG};

use super::{components::*, pathfinder};

const THRESHOLD: f32 = 100.0;
const UPP_THRESHOLD: f32 = THRESHOLD * 2.0;
const CIV_MS: f32 = 30.0;
const PROJ_V: f32 = 150.0;
const HUNTER_TIMER: f32 = 0.5;
const HUNTER_MAXSPEED: f32 = 50.0;
const HUNTER_ACCEL: f32 = 450.0;

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
        CollisionGroups::new(
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
        Name::new("Hunter"),
        SpriteBundle {
            texture: asset_server.load("sprites/4el.png"),
            transform: Transform::from_xyz(-40., -40., 0.),
            ..default()
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(5.),
        Hunter,
        NpcVelAccum {v: Vec2::ZERO},
        NpcPath {path: None},
        KinematicCharacterController::default(),
        CollisionGroups::new(
            Group::from_bits(NPC_CG).unwrap(),
            Group::from_bits(PLAYER_CG & STRUCTURES_CG).unwrap()
        ),
        HunterTimer { timer: Timer::new(Duration::from_secs_f32(HUNTER_TIMER), TimerMode::Repeating) },
        NpcState::Chase,
    ));
}

pub fn manage_hunters(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut hunters_data: Query<(&Transform, &mut KinematicCharacterController,
        &mut NpcVelAccum, &mut NpcPath, &mut HunterTimer, &mut NpcState), Without<Player>>,
    player_data: Query<(&Transform, &PlayerController), With<Player>>,
    transformer: Res<TransformToGrid>,
    trespassable: Res<TrespassableCells>,
    time: Res<Time>,
    mut prev_player_ipos: Local<IVec2>,
    mut gizmos: Gizmos,
) {
    let player_data = player_data.single();
    let player_pos = player_data.0.translation.xy();
    let player_ipos = transformer.from_world_i32(player_pos);
    let player_vel = player_data.1.accumulated_velocity;
    let dt = time.delta_seconds();
    for (hunter_transform, mut hunter_controller,
        mut vel_accum , mut hunter_path,
        mut hunter_timer, mut hunter_state) in hunters_data.iter_mut() {
        let hunter_pos = hunter_transform.translation.xy();
        let hunter_ipos = transformer.from_world_i32(hunter_pos);

        match *hunter_state {
            NpcState::Attack => {
            hunter_timer.timer.tick(Duration::from_secs_f32(dt));
            if hunter_timer.timer.finished() { // todo: shoot only if raycast
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
                        CollisionGroups::new(
                            Group::from_bits(BULLET_CG).unwrap(),
                            Group::from_bits(PLAYER_CG).unwrap()
                        ),
                        LockedAxes::ROTATION_LOCKED_Z,
                        Velocity {
                            linvel: PROJ_V * dir,
                            angvel: 0.0,
                        },
                        DespawnTimer { timer: Timer::new(Duration::from_secs(6), TimerMode::Once) },
                        Projectile,
                        ActiveEvents::COLLISION_EVENTS,
                        Sleeping::disabled(),
                    ));
                }
            }
            let dist = player_pos.distance(hunter_pos);
            if dist < THRESHOLD {
                *hunter_state = NpcState::Escape;
            } else if dist < UPP_THRESHOLD {
                *hunter_state = NpcState::Chase;
            }
            }
            NpcState::Chill => {

            }
            NpcState::Dead => {

            }
            state => {
                if player_ipos != *prev_player_ipos {
                    hunter_path.path = pathfinder(hunter_ipos, player_ipos, &trespassable, &transformer, state);
                } else {
                    let mut del = false;
                    if let Some(path) = &mut hunter_path.path {
                        if hunter_ipos == path[1] {
                            path.remove(0);
                        }
                        if path.len() < 2 {
                            del = true;
                        }
                    }
                    if del {
                        hunter_path.path = None;
                    }
                }
                
                if let Some(path) = &hunter_path.path {
                    for id in 0..path.len() - 1 {
                        let p0 = transformer.to_world(path[id]);
                        let p1 = transformer.to_world(path[id + 1]);
                        gizmos.line_2d(p0, p1, Color::Srgba(RED))
                    }
                    let move_dir = transformer.to_world(path[1]) - hunter_transform.translation.xy();
                    
                    vel_accum.v = vel_accum.v.move_towards(move_dir.normalize_or_zero() * HUNTER_MAXSPEED, dt * HUNTER_ACCEL);
                    if vel_accum.v.length() > HUNTER_MAXSPEED {
                        vel_accum.v = vel_accum.v.normalize() * HUNTER_MAXSPEED
                    }
                    hunter_controller.translation = Some(vel_accum.v * dt);
                } else {
                    *hunter_state = NpcState::Attack;
                }
            }
        }
        
    }
    *prev_player_ipos = player_ipos;
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
    mut player: Query<(Entity, &mut Player)>,
    projectiles: Query<&Projectile>,
) {
    let (player_entity, player) = player.single_mut();
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(reciever_entity, sender_entity, _) = collision_event {
            println!("{:?} {:?}", reciever_entity, sender_entity);
            if let Ok(_) = projectiles.get(*sender_entity) { // todo: rm if process only proj
                if *reciever_entity == player_entity {
                    println!("hit");
                }
                commands.entity(*sender_entity).despawn();
            } else if let Ok(_) = projectiles.get(*reciever_entity) { // todo: rm if process only proj
                if *sender_entity == player_entity {
                    println!("hit");
                }
                commands.entity(*reciever_entity).despawn();
            }
        }
    }
}