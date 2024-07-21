use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{BULLET_CG, PLAYER_CG};

use super::components::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/telega.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(10., 10.),
        GravityScale(0.),
        Player,
        LockedAxes::ROTATION_LOCKED_Z,
        Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        },
        Damping {
            linear_damping: 0.2,
            angular_damping: 0.0,
        },
        SolverGroups::new(
            Group::from_bits(PLAYER_CG).unwrap(),
            Group::ALL,
        ),
        ActiveEvents::COLLISION_EVENTS,
        Name::new("Player"),
    ));
}

pub fn player_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_data: Query<(&mut Damping, &mut Velocity, &Transform), With<Player>>,
    mut camera_transform: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_data = &mut player_data.single_mut();
    let (player_damping, player_velocity, player_transform) = player_data;
    let dt = time.delta_seconds();
    let mut direction = Vec2::ZERO;
    let mut ms = 100.;
    let mut pressed = false;
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= Vec2::X;
        pressed = true;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec2::X;
        pressed = true;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec2::Y;
        pressed = true;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= Vec2::Y;
        pressed = true;
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        ms *= 2.;
    }
    if pressed {
        player_damping.linear_damping = 0.0;
    } else {
        player_damping.linear_damping = 5.;
    }
    // *player_translation += (direction * ms * dt).extend(0.);
    player_velocity.linvel += direction * ms * dt;
    camera_transform.single_mut().translation = player_transform.translation;
}