use std::{f32::consts::PI, time::Duration};

use bevy::{color::palettes::css::{BLUE, RED}, math::uvec2, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    characters::animation::*, core::functions::TextureAtlasLayoutHandles, map::{plugin::{EntitySpawner, TrespassableCells}, tilemap::{Structure, TransformToGrid}}, player::{components::Player, systems::{PlayerController, BULLET_CG, NPC_CG, PLAYER_CG, STRUCTURES_CG}}, systems::DayCycle
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
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec2,
    layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
) {
    let entity = spawn_civilian_animation_bundle(&mut commands, asset_server, layout_handles);
    commands.entity(entity).insert((
        TransformBundle::from_transform(Transform::from_translation(pos.extend(0.))),
        RigidBody::Dynamic,
        Velocity::zero(),
        Civilian,
        Sleeping::disabled(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::ball(4.5),
        CollisionGroups::new(
            Group::from_bits(NPC_CG).unwrap(),
            Group::from_bits(PLAYER_CG).unwrap()
        ),
        NpcVelAccum {v: Vec2::ZERO},
        NpcPath {path: None},
        NpcState::Chill,
        ChillTimer {timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating)},
        AttackTimer {timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating)},
    ));
}

pub fn manage_civilians(
    mut civilians_data: Query<(&Transform, &mut Velocity, &mut NpcVelAccum, &mut NpcPath, &mut NpcState,
        &mut ChillTimer, &mut AnimationController, &mut AttackTimer), With<Civilian>>,
    mut player_data: Query<(&Transform, Entity, &mut Player)>,
    time: Res<Time>,
    day_cycle: Res<DayCycle>,
    transformer: Res<TransformToGrid>,
    trespassable: Res<TrespassableCells>,
    mut gizmos: Gizmos,
    rapier_context: Res<RapierContext>,
) {
    let (player_transform, player_entity, mut player) = player_data.single_mut();
    let player_pos = player_transform.translation.xy();
    let player_ipos = transformer.from_world_i32(player_pos);
    let dt = time.delta_seconds();
    let mut rng = thread_rng();
    for (civ_transform, mut civ_controller,
        mut vel_accum , mut civ_path,
        mut civ_state, mut chill_timer,
        mut animation_controller, mut attack_timer) in civilians_data.iter_mut() {
        let civ_pos = civ_transform.translation.xy();
        let civ_ipos = transformer.from_world_i32(civ_pos);
        let direction = player_pos - civ_pos;
        let length = direction.length();
        let mut player_in_sight = false;
        if let Some(last_seen_entity) = raycast(civ_pos, direction / length, length, &rapier_context) {
        if last_seen_entity == player_entity && length < SPOT_DIST {
            player_in_sight = true;
        }
        println!("{:?} {}", civ_state, player_in_sight);
        match *civ_state {
            NpcState::Look => {},
            NpcState::Dead => {},
            NpcState::Attack => {
                if attack_timer.timer.elapsed_secs() == 0. {
                    animation_controller.play_civil_attack();
                }
                attack_timer.timer.tick(Duration::from_secs_f32(dt));
                if attack_timer.timer.finished() {
                    if player_pos.distance(civ_pos) < 16. {
                        player.hp -= 10;
                    }
                    *civ_state = NpcState::Chase;
                    attack_timer.timer.set_elapsed(Duration::from_secs(0))
                }
            },
            state => { // esc cha chi
                let mut stop = false;
                if state == NpcState::Chill {
                    animation_controller.disarm();
                    
                    animation_controller.play_idle_priority(1);

                    if civ_path.path.is_none() {
                        chill_timer.timer.tick(Duration::from_secs_f32(dt));
                        if chill_timer.timer.finished() {
                            let end = civ_ipos + IVec2::new(rng.gen_range(-4..4), rng.gen_range(-4..4));
                            if trespassable.is_trespassable(&end) {
                                civ_path.path = pathfinder(civ_ipos, end, &trespassable, &transformer, state, false);
                            }
                        }
                    }
                    if player_in_sight {
                        if day_cycle.is_night {
                            // todo: play "!" anim
                            *civ_state = NpcState::Escape;
                        } else {
                            // todo: play "!" anim
                            *civ_state = NpcState::Chase;
                        }
                    }
                } else if state == NpcState::Escape {
                    animation_controller.disarm();
                    civ_path.path = pathfinder(civ_ipos, player_ipos, &trespassable, &transformer, state, false);
                    if !day_cycle.is_night {
                        if player_in_sight {
                            *civ_state = NpcState::Chase;
                        } else {
                            *civ_state = NpcState::Chill;
                        }
                    }
                } else { // chase
                    // todo: play "*" anim
                    animation_controller.arm();
                    civ_path.path = pathfinder(civ_ipos, player_ipos, &trespassable, &transformer, state, false);
                    if player_in_sight {
                        if day_cycle.is_night {
                            *civ_state = NpcState::Escape;
                        }
                    } else {
                        *civ_state = NpcState::Chill;
                    }
                    if player_pos.distance(civ_pos) < 16. {
                        *civ_state = NpcState::Attack;
                        stop = true;
                    }
                    // println!("{:?}", civ_path.path);
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
                        animation_controller.play_walk_unlooped();
                    } else {
                        animation_controller.play_idle_priority(1);
                    }
                    vel_accum.v = vel_accum.v.move_towards(move_dir.normalize_or_zero() * CIV_MAXSPEED, dt * CIV_ACCEL);
                    if vel_accum.v.length() > CIV_MAXSPEED {
                        vel_accum.v = vel_accum.v.normalize() * CIV_MAXSPEED
                    }
                    civ_controller.linvel = vel_accum.v;
                } else {
                    civ_controller.linvel = Vec2::ZERO;
                    if civ_pos.distance(player_pos) > THRESHOLD {
                        *civ_state = NpcState::Chill
                    } else {
                        if day_cycle.is_night {
                            *civ_state = NpcState::Escape
                        } else {
                            *civ_state = NpcState::Chase
                        }
                    }
                }
                if stop {
                    warn!("OVERRIDE!");
                    civ_controller.linvel = Vec2::ZERO;
                    animation_controller.play_idle_priority(1);
                }
            }
        }
    }
    }
}


pub fn spawn_hunter(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec2,
    layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
) {
    let entity = spawn_hunter_animation_bundle(commands, asset_server, layout_handles);
    commands.entity(entity).insert((
        (
            Name::new("Hunter"),
            RigidBody::Dynamic,
            TransformBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            VisibilityBundle::default(),
            Collider::ball(4.5),
            Sleeping::disabled(),
        ),
        Hunter,
        LockedAxes::ROTATION_LOCKED_Z,
        NpcVelAccum {v: Vec2::ZERO},
        NpcPath {path: None},
        Velocity::zero(),
        CollisionGroups::new(
            Group::from_bits(NPC_CG).unwrap(),
            Group::from_bits(PLAYER_CG).unwrap(),
        ),
        HunterTimer { timer: Timer::new(Duration::from_secs_f32(HUNTER_TIMER), TimerMode::Repeating) },
        NpcState::Chill,
        ChillTimer {timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating)},
        PlayerLastPos {pos: IVec2::ZERO},
    )).with_children(|commands| {commands.spawn((
        PartType::Body{variant: 0, variants: 1},
        SpriteBundle{
            texture: asset_server.load("hunter/hunter.png"),
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
    asset_server: Res<AssetServer>,
    mut hunters_data: Query<(&Transform, &mut Velocity,
        &mut NpcVelAccum, &mut NpcPath, &mut HunterTimer, &mut NpcState,
        &mut ChillTimer, &mut AnimationController, &mut PlayerLastPos), Without<Player>>,
    player_data: Query<(&Transform, &PlayerController, Entity), With<Player>>,
    transformer: Res<TransformToGrid>,
    trespassable: Res<TrespassableCells>,
    mut gizmos: Gizmos,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut atlas_handles: ResMut<TextureAtlasLayoutHandles>
) {
    let player_data = player_data.single();
    let player_pos = player_data.0.translation.xy();
    let player_ipos = transformer.from_world_i32(player_pos);
    let player_vel = player_data.1.accumulated_velocity;
    let player_entity = player_data.2;
    let dt = time.delta_seconds();
    for (hunter_transform, mut hunter_controller,
        mut vel_accum , mut hunter_path,
        mut hunter_timer, mut hunter_state, mut chill_timer,
        mut animation_controller, mut player_last_pos) in hunters_data.iter_mut() {
        hunter_controller.linvel = Vec2::ZERO;
        let hunter_pos = hunter_transform.translation.xy();
        let hunter_ipos = transformer.from_world_i32(hunter_pos);
        let direction = player_pos - hunter_pos;
        let length = direction.length();
        let mut player_in_sight = false;
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
                    let throwable_variant = rand::thread_rng().gen_range(0..4);
                    let e = match throwable_variant {
                        0 => {commands.spawn(crate::stuff::animated_fork_bundle(&asset_server, &mut atlas_handles)).id()},
                        1 => {commands.spawn(crate::stuff::animated_knife_bundle(&asset_server, &mut atlas_handles)).id()},
                        2 => {commands.spawn(crate::stuff::animated_garlic_bundle(&asset_server, &mut atlas_handles)).id()},
                        _ => {commands.spawn(crate::stuff::stake_bundle(&asset_server, &mut atlas_handles, dir)).id()},
                    };
                    commands.entity(e).insert((
                        Transform::from_translation(hunter_pos.extend(0.)).with_rotation(Quat::from_rotation_z(if throwable_variant != 3 {0.} else {dir.to_angle() + PI * 0.75})),
                        RigidBody::Dynamic,
                        Collider::cuboid(3., 3.),
                        CollisionGroups::new(
                            Group::from_bits(BULLET_CG).unwrap(),
                            Group::from_bits(PLAYER_CG | STRUCTURES_CG).unwrap()
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
                    animation_controller.play_idle_priority(1);
                    if player_in_sight {
                        *hunter_state = NpcState::Chase;
                        // todo: add "!" anim
                    } else {
                        if hunter_path.path.is_none() {
                            let mut rng = thread_rng();
                            chill_timer.timer.tick(Duration::from_secs_f32(dt));
                            if chill_timer.timer.finished() {
                                let end = hunter_ipos + IVec2::new(rng.gen_range(-4..4), rng.gen_range(-4..4));
                                if trespassable.is_trespassable(&end) {
                                    hunter_path.path = pathfinder(hunter_ipos, end, &trespassable, &transformer, state, true);
                                }
                            }
                        }
                    }
                } else if state == NpcState::Look {
                    if player_in_sight {
                        *hunter_state = NpcState::Chase;
                        // todo: add "!" anim
                    } else {
                        hunter_path.path = pathfinder(hunter_ipos, player_last_pos.pos, &trespassable, &transformer, state, true);
                        if hunter_path.path.is_none() {
                            // todo: add "?" anim
                            *hunter_state = NpcState::Chill;
                        }
                    }
                    
                } else { // chase & escape
                    if player_in_sight {
                        hunter_path.path = pathfinder(hunter_ipos, player_ipos, &trespassable, &transformer, state, true);
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
                        animation_controller.play_walk_unlooped();
                    } else {
                        animation_controller.play_idle_priority(1);
                    }
                    
                    vel_accum.v = vel_accum.v.move_towards(move_dir.normalize_or_zero() * HUNTER_MAXSPEED, dt * HUNTER_ACCEL);
                    if vel_accum.v.length() > HUNTER_MAXSPEED {
                        vel_accum.v = vel_accum.v.normalize() * HUNTER_MAXSPEED
                    }
                    hunter_controller.linvel = vel_accum.v;
                }
            }
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

pub fn process_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<(Entity, &mut Player)>,
    mut hunters: Query<&mut NpcState, (With<Hunter>, Without<Civilian>)>,
    mut civilians: Query<&mut NpcState, With<Civilian>>,
    projectiles: Query<&Projectile>,
    structures: Query<&Structure>,
    day_cycle: Res<DayCycle>,
) {
    let (player_entity, mut player) = player.single_mut();
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(reciever_entity, sender_entity, _) = collision_event {
            // println!("{:?}", collision_event);
            // player appears to always be reciever
            let sender_entity = *sender_entity;
            if let Ok(_) = projectiles.get(sender_entity) {
                if *reciever_entity == player_entity {
                    // todo: play hurt anim
                    player.hp -= 10;
                }
                commands.entity(sender_entity).despawn();
            } else if let Ok(mut state) = civilians.get_mut(sender_entity) {
                if day_cycle.is_night {
                    // kill civilian
                    *state = NpcState::Dead;
                } else {
                    if *reciever_entity == player_entity {
                        // todo: play hurt anim
                        player.hp -= 10;
                    }
                }
            } else if let Ok(mut state) = hunters.get_mut(sender_entity) {
                if day_cycle.is_night {
                    // kill hunter
                    *state = NpcState::Dead;
                } else {
                    if *reciever_entity == player_entity {
                        // todo: play hurt anim
                        player.hp -= 10;
                    }
                }
            } else if let Ok(_) = structures.get(sender_entity) {
                commands.entity(player_entity).remove::<Sensor>();
            }
        }
    }
}

pub fn entity_spawner(
    mut commands: Commands,
    mut spawners: Query<(&mut EntitySpawner, &GlobalTransform)>,
    mut npcs_on_map: ResMut<NpcsOnMap>,
    mut layout_handles: ResMut<TextureAtlasLayoutHandles>,
    asset_server: Res<AssetServer>,
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
                    spawn_civilian(&mut commands, &asset_server, spawner_pos, &mut layout_handles);
                    npcs_on_map.civilians += 1;
                }
            } else {
                if npcs_on_map.hunters < 1 {
                    spawn_hunter(&mut commands, &asset_server, spawner_pos, &mut layout_handles);
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