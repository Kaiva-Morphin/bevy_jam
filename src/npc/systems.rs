use std::time::Duration;

use bevy::{color::palettes::css::{BLUE, RED}, math::uvec2, prelude::*};
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    characters::animation::*,
    map::{plugin::{EntitySpawner, TrespassableCells}, tilemap::TransformToGrid},
    player::{components::Player, systems::{PlayerController, BULLET_CG, NPC_CG, PLAYER_CG, STRUCTURES_CG}},
};

use super::{components::*, pathfinder};

const SPOT_DIST: f32 = 200.0;
const THRESHOLD: f32 = 100.0;
const UPP_THRESHOLD: f32 = THRESHOLD * 2.0;
const CIV_MAXSPEED: f32 = 40.0;
const CIV_ACCEL: f32 = 350.0;
const PROJ_V: f32 = 150.0;
const HUNTER_TIMER: f32 = 0.5;
const HUNTER_MAXSPEED: f32 = 50.0;
const HUNTER_ACCEL: f32 = 450.0;

pub fn spawn_civilian(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    pos: Vec2,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/box.png"),
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            ..default()
        },
        Civilian,
        CollisionGroups::new(
            Group::from_bits(NPC_CG).unwrap(),
            Group::from_bits(PLAYER_CG | STRUCTURES_CG).unwrap()
        ),
        RigidBody::KinematicPositionBased,
        Collider::ball(5.),
        NpcVelAccum {v: Vec2::ZERO},
        NpcPath {path: None},
        KinematicCharacterController::default(),
        NpcState::Chill,
        ChillTimer {timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating)},
        Name::new("Civilian"),
    ));
}

pub fn manage_civilians(
    mut civilians_data: Query<(&Transform, &mut KinematicCharacterController,
        &mut NpcVelAccum, &mut NpcPath, &mut NpcState, &mut ChillTimer), With<Civilian>>,
    player_transform: Query<&Transform, With<Player>>,
    time: Res<Time>,
    transformer: Res<TransformToGrid>,
    trespassable: Res<TrespassableCells>,
    mut gizmos: Gizmos,
    mut prev_player_ipos: Local<IVec2>,
) {
    let player_pos = player_transform.single().translation.xy();
    let player_ipos = transformer.from_world_i32(player_pos);
    let dt = time.delta_seconds();
    let mut rng = thread_rng();
    for (civ_transform, mut civ_controller,
        mut vel_accum , mut civ_path,
        mut civ_state, mut chill_timer) in civilians_data.iter_mut() {
        let civ_pos = civ_transform.translation.xy();
        let civ_ipos = transformer.from_world_i32(civ_pos);
        match *civ_state {
            NpcState::Attack => todo!(),
            NpcState::Dead => todo!(),
            NpcState::Chase => todo!(),
            state => {
                if state == NpcState::Chill {
                    if civ_path.path.is_none() {
                        chill_timer.timer.tick(Duration::from_secs_f32(dt));
                        if chill_timer.timer.finished() {
                            let end = civ_ipos + IVec2::new(rng.gen_range(-4..4), rng.gen_range(-4..4));
                            if trespassable.is_tresspassable(&end) {
                                civ_path.path = pathfinder(civ_ipos, end, &trespassable, &transformer, state);
                            }
                        }
                    }
                } else {
                    if player_ipos != *prev_player_ipos {
                        civ_path.path = pathfinder(civ_ipos, player_ipos, &trespassable, &transformer, state);
                    }
                }
                
                let mut del = false;
                if let Some(path) = &mut civ_path.path {
                    if civ_ipos == path[1] {
                        path.remove(0);
                    }
                    if path.len() < 2 {
                        del = true;
                    }
                }
                if del {
                    civ_path.path = None;
                }
                
                if let Some(path) = &civ_path.path {
                    for id in 0..path.len() - 1 {
                        let p0 = transformer.to_world(path[id]);
                        let p1 = transformer.to_world(path[id + 1]);
                        gizmos.line_2d(p0, p1, Color::Srgba(RED))
                    }
                    let move_dir = transformer.to_world(path[1]) - civ_pos;

                    // if move_dir.x.abs() < 0.1 { // x axis is priotirized 
                    //     if move_dir.y.abs() > 0.1 {
                    //         if move_dir.y.is_sign_positive(){animation_controller.turn_up()}
                    //         if move_dir.y.is_sign_negative(){animation_controller.turn_down()}
                    //     }
                    // } else {
                    //     if move_dir.x.is_sign_positive(){animation_controller.turn_right()}
                    //     if move_dir.x.is_sign_negative(){animation_controller.turn_left()}
                    // }
                    // if vel_accum.v.length() > 0.1 {
                    //     animation_controller.play_walk();
                    // } else {
                    //     animation_controller.play_idle_priority(1);
                    // }

                    vel_accum.v = vel_accum.v.move_towards(move_dir.normalize_or_zero() * CIV_MAXSPEED, dt * CIV_ACCEL);
                    if vel_accum.v.length() > CIV_MAXSPEED {
                        vel_accum.v = vel_accum.v.normalize() * CIV_MAXSPEED
                    }
                    civ_controller.translation = Some(vel_accum.v * dt);
                } else {
                    if civ_pos.distance(player_pos) > THRESHOLD {
                        *civ_state = NpcState::Chill
                    } else {
                        *civ_state = NpcState::Escape
                    }
                }
            }
        }
    }
    *prev_player_ipos = player_ipos;
}

pub fn spawn_hunter(
    commands: &mut Commands,
    asset_server: &mut ResMut<AssetServer>,
    pos: Vec2,
) {
    commands.spawn((
        Name::new("Hunter"),
        RigidBody::KinematicPositionBased,
        TransformBundle::from_transform(Transform::from_translation(pos.extend(0.))),
        VisibilityBundle::default(),
        Collider::ball(5.),
        Hunter,
        NpcVelAccum {v: Vec2::ZERO},
        NpcPath {path: None},
        KinematicCharacterController::default(),
        CollisionGroups::new(
            Group::from_bits(NPC_CG).unwrap(),
            Group::from_bits(PLAYER_CG | STRUCTURES_CG).unwrap()
        ),
        HunterTimer { timer: Timer::new(Duration::from_secs_f32(HUNTER_TIMER), TimerMode::Repeating) },
        NpcState::Chase,
        AnimationController::default(),
        ChillTimer {timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating)},
        PlayerLastPos {pos: IVec2::ZERO},
    )).with_children(|commands| {commands.spawn((
        SpriteBundle{
            texture: asset_server.load("hunter.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(16, 20), 7, 3, Some(uvec2(1, 1)), None)),
            index: 2
        }
    ));});
}

pub fn manage_hunters(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut hunters_data: Query<(&Transform, &mut KinematicCharacterController,
        &mut NpcVelAccum, &mut NpcPath, &mut HunterTimer, &mut NpcState,
        &mut ChillTimer, &mut AnimationController, &mut PlayerLastPos), Without<Player>>,
    player_data: Query<(&Transform, &PlayerController, Entity), With<Player>>,
    transformer: Res<TransformToGrid>,
    trespassable: Res<TrespassableCells>,
    mut prev_player_ipos: Local<IVec2>,
    mut gizmos: Gizmos,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    let player_data = player_data.single();
    let player_pos = player_data.0.translation.xy();
    let player_ipos = transformer.from_world_i32(player_pos);
    let player_vel = player_data.1.accumulated_velocity;
    let player_entity = player_data.2;
    let mut player_in_sight = false;
    let dt = time.delta_seconds();
    for (hunter_transform, mut hunter_controller,
        mut vel_accum , mut hunter_path,
        mut hunter_timer, mut hunter_state, mut chill_timer,
        mut animation_controller, mut player_last_pos) in hunters_data.iter_mut() {
        let hunter_pos = hunter_transform.translation.xy();
        let hunter_ipos = transformer.from_world_i32(hunter_pos);
        let direction = player_pos - hunter_pos;
        let length = direction.length();
        if let Some(last_seen_entity) = raycast(hunter_pos, direction / length, length, &rapier_context) {
        if last_seen_entity == player_entity && length < SPOT_DIST{
            player_in_sight = true;
        }
        gizmos.line_2d(hunter_pos, player_pos, Color::Srgba(BLUE));

        match *hunter_state {
            NpcState::Attack => {
                hunter_timer.timer.tick(Duration::from_secs_f32(dt));
                let dir = player_pos - hunter_pos;
                if dir.x.abs() > dir.y.abs() {
                    if dir.x > 0. {
                        animation_controller.turn_right()
                    } else {
                        animation_controller.turn_left()
                    }
                } else {
                    if dir.y > 0. {
                        animation_controller.turn_up()
                    } else {
                        animation_controller.turn_down()
                    }
                }
                if player_in_sight {
                if hunter_timer.timer.finished() {
                animation_controller.play_hunter_throw();
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
                        Collider::cuboid(3., 3.),
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
                        Sensor,
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
            } else {
            *hunter_state = NpcState::Look;
            player_last_pos.pos = player_ipos;
            }
            }
            NpcState::Dead => {

            }
            state => {
                if state == NpcState::Chill {
                    if player_in_sight {
                        *hunter_state = NpcState::Chase;
                        // todo: add "!" anim
                    } else {
                        if hunter_path.path.is_none() {
                            let mut rng = thread_rng();
                            chill_timer.timer.tick(Duration::from_secs_f32(dt));
                            if chill_timer.timer.finished() {
                                let end = hunter_ipos + IVec2::new(rng.gen_range(-4..4), rng.gen_range(-4..4));
                                if trespassable.is_tresspassable(&end) {
                                    hunter_path.path = pathfinder(hunter_ipos, end, &trespassable, &transformer, state);
                                }
                            }
                        }
                    }
                } else if state == NpcState::Look {
                    if player_in_sight {
                        *hunter_state = NpcState::Chase;
                        // todo: add "!" anim
                    } else {
                        hunter_path.path = pathfinder(hunter_ipos, player_last_pos.pos, &trespassable, &transformer, state);
                        if hunter_path.path.is_none() {
                            // todo: add "?" anim
                            *hunter_state = NpcState::Chill;
                        }
                    }
                    
                } else { // chase & escape
                    if player_in_sight {
                        if player_ipos != *prev_player_ipos {
                            hunter_path.path = pathfinder(hunter_ipos, player_ipos, &trespassable, &transformer, state);
                        }
                        if hunter_path.path.is_none() {
                        *hunter_state = NpcState::Attack;
                        }
                    } else {
                        *hunter_state = NpcState::Look;
                        player_last_pos.pos = player_ipos;
                    }
                }
                
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
                
                if let Some(path) = &hunter_path.path {
                    for id in 0..path.len() - 1 {
                        let p0 = transformer.to_world(path[id]);
                        let p1 = transformer.to_world(path[id + 1]);
                        gizmos.line_2d(p0, p1, Color::Srgba(RED))
                    }
                    let move_dir = transformer.to_world(path[1]) - hunter_pos;

                    if move_dir.x.abs() < 0.1 { // x axis is priotirized 
                        if move_dir.y.abs() > 0.1 {
                            if move_dir.y.is_sign_positive(){animation_controller.turn_up()}
                            if move_dir.y.is_sign_negative(){animation_controller.turn_down()}
                        }
                    } else {
                        if move_dir.x.is_sign_positive(){animation_controller.turn_right()}
                        if move_dir.x.is_sign_negative(){animation_controller.turn_left()}
                    }
                    if vel_accum.v.length() > 0.1 {
                        animation_controller.play_walk();
                    } else {
                        animation_controller.play_idle_priority(1);
                    }

                    vel_accum.v = vel_accum.v.move_towards(move_dir.normalize_or_zero() * HUNTER_MAXSPEED, dt * HUNTER_ACCEL);
                    if vel_accum.v.length() > HUNTER_MAXSPEED {
                        vel_accum.v = vel_accum.v.normalize() * HUNTER_MAXSPEED
                    }
                    hunter_controller.translation = Some(vel_accum.v * dt);
                }
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
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut player: Query<(Entity, &mut Player)>,
    projectiles: Query<&Projectile>,
) {
    let (player_entity, player) = player.single_mut();
    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
    for collision_event in collision_events.read() {
        println!("{:?}", collision_event);
        if let CollisionEvent::Started(reciever_entity, sender_entity, _) = collision_event {
            // println!("{:?} {:?}", reciever_entity, sender_entity);
            if let Ok(_) = projectiles.get(*sender_entity) { // todo: rm if process only proj
                if *reciever_entity == player_entity {
                    println!("PLAYER RECIEVED")
                }
                commands.entity(*sender_entity).despawn();
            }
        }
    }
}

pub fn entity_spawner(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut spawners: Query<(&mut EntitySpawner, &GlobalTransform)>,
    mut npcs_on_map: ResMut<NpcsOnMap>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let mut rand = thread_rng();
    for (mut spawner, spawner_gpos) in spawners.iter_mut() {
        spawner.timer.tick(Duration::from_secs_f32(dt));
        if spawner.timer.finished() {
            let spawner_pos = spawner_gpos.translation().xy();
            if rand.gen_bool(0.5) {
                if npcs_on_map.civilians < 1 {
                    spawn_civilian(&mut commands, &mut asset_server, spawner_pos);
                    npcs_on_map.civilians += 1;
                }
            } else {
                if npcs_on_map.hunters < 10 {
                    spawn_hunter(&mut commands, &mut asset_server, spawner_pos);
                    npcs_on_map.hunters += 1;
                }
            }
        }
    }
    // if let Ok(t) = spawner.get_single() {
    //     println!("{:?}", transformer.from_world_i32(t.translation().xy()))
    // }
}

fn raycast(
    origin: Vec2,
    dir: Vec2,
    max_toi: f32,
    rapier_context: &Res<RapierContext>,
) -> Option<Entity> {
    let solid = true;
    let filter = QueryFilter::default();
    let filter = filter.groups(CollisionGroups::new(
        Group::all(),
        Group::from_bits(STRUCTURES_CG | PLAYER_CG).unwrap())
    );
    if let Some((entity, _)) = rapier_context.cast_ray(
        origin, dir, max_toi, solid, filter
    ) {
        return Some(entity);
    } else {
        return None;
    }
}