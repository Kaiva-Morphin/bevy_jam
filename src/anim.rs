pub mod core;
pub mod player;
pub mod npc;
pub mod map;
pub mod systems;
pub mod stuff;
pub mod characters;
pub mod sounds;


use crate::player::components::Player;
use core::{camera::plugin::EnhancedCameraPlugin, functions::TextureAtlasLayoutHandles};
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;
use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use characters::plugin::CharacterAnimationPlugin;
use map::plugin::TileMapPlugin;
use npc::NPCPlugin;
use player::PlayerPlugin;
use sounds::AudioPlugin;
use stuff::{simple_anim_update, spawn_follow_blood_particle, update_blood_particles};
use systems::*;
use core::despawn_lifetime::DespawnTimer;
use crate::characters::animation::CivilianAnims;

use bevy_easings::{Ease, EaseFunction, EasingType};
use characters::animation::{spawn_civilian_animation_bundle, spawn_player_animation_bundle, AnimationController};
use rand::Rng;
use stuff::*;
use systems::GameState;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
    ))
    .add_plugins((
        CharacterAnimationPlugin,
    ));
    app.insert_state(GameState::InGame);
    app.add_systems(PreUpdate, (update, simple_anim_update));
    app.add_systems(Startup, setup);
    app.run();
}

#[derive(Component)]
pub struct PreviewCharacter;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layout_handles: ResMut<TextureAtlasLayoutHandles>
){
    let entity = spawn_player_animation_bundle(&mut commands, &asset_server, &mut layout_handles);
    commands.entity(entity).insert(PreviewCharacter);
}



fn update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut controller: Query<(&mut AnimationController, Entity), With<PreviewCharacter>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layout_handles: ResMut<TextureAtlasLayoutHandles>
){
    
    spawn_angry_particle(&mut commands, &mut layout_handles, &asset_server, vec3(-10., 10., 0.));
    
    if keyboard.just_pressed(KeyCode::Space){
        let entity = spawn_civilian_animation_bundle(&mut commands, &asset_server, &mut layout_handles);
        commands.entity(entity).insert(PreviewCharacter);
        for (_, e) in controller.iter(){
            commands.entity(e).despawn_recursive();
        }
        return;
    }
    for (mut controller, _) in controller.iter_mut(){
        if keyboard.pressed(KeyCode::KeyA){controller.turn_left()};
        if keyboard.pressed(KeyCode::KeyD){controller.turn_right()};
        if keyboard.pressed(KeyCode::KeyS){controller.turn_down()};
        if keyboard.pressed(KeyCode::KeyW){controller.turn_up()};
        if keyboard.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD]){
            controller.play_walk_unlooped();
        } else {
            controller.play_idle_priority(1);
        }
        if keyboard.just_pressed(KeyCode::Digit1){
            controller.arm();
        }
        if keyboard.just_pressed(KeyCode::Digit2){
            controller.disarm();
        }
        if keyboard.just_pressed(KeyCode::Digit3){
            controller.play_civil_attack();
        }
        if keyboard.just_pressed(KeyCode::Digit4){
            controller.play_hurt();
        }

        if keyboard.just_pressed(KeyCode::Digit5){
            let max_offset = 7.;
            let start = vec3(
                rand::random::<f32>() * max_offset * 2. - max_offset,
                rand::random::<f32>() * max_offset * 2. - max_offset,
                0.
            );

            let flipped = rand::thread_rng().gen::<bool>();
            commands.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                DespawnTimer::seconds(1.),
            ))
            .insert(Transform::from_translation(vec3(0., 10., 8.) + start))
            .with_children(|commands| {
                commands.spawn((
                    Name::new("Particle"),
                    emotion_bundle(&asset_server, &mut layout_handles, rand::thread_rng().gen_range(0..3)),
                    Transform::from_translation(vec3(0., 0., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(Vec3::splat(0.5) * vec3(if flipped{-1.} else {1.}, 1., 1.))
                        .ease_to(
                            Transform::from_translation(vec3(rand::thread_rng().gen::<f32>() * 3. - 1.5, 4. + rand::thread_rng().gen::<f32>() * 5., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(vec3(if flipped{-1.} else {1.}, 1., 1.)),
                            EaseFunction::ExponentialOut,
                            EasingType::Once {
                                duration: std::time::Duration::from_secs(1),
                            },
                        )
                )).insert(
                    Sprite{flip_x: flipped, ..default()}.ease_to(
                        Sprite { color: Color::Srgba(Srgba::new(1., 1., 1., 0.)), flip_x: flipped,..default() },
                        EaseFunction::ExponentialOut,
                        EasingType::Once {
                            duration: std::time::Duration::from_secs(1),
                        },
                    )
                );
            });

            commands.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                DespawnTimer::seconds(1.),
            ))
            .insert(Transform::from_translation(vec3(10., 20., 8.) + start))
            .with_children(|commands| {
                commands.spawn((
                    Name::new("Particle"),
                    emotion_bundle(&asset_server, &mut layout_handles, rand::thread_rng().gen_range(0..3) + 3),
                    Transform::from_translation(vec3(0., 0., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(Vec3::splat(0.5) * vec3(if flipped{-1.} else {1.}, 1., 1.))
                        .ease_to(
                            Transform::from_translation(vec3(rand::thread_rng().gen::<f32>() * 3. - 1.5, 4. + rand::thread_rng().gen::<f32>() * 5., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(vec3(if flipped{-1.} else {1.}, 1., 1.)),
                            EaseFunction::ExponentialOut,
                            EasingType::Once {
                                duration: std::time::Duration::from_secs(1),
                            },
                        )
                )).insert(
                    Sprite{..default()}.ease_to(
                        Sprite { color: Color::Srgba(Srgba::new(1., 1., 1., 0.)), ..default() },
                        EaseFunction::ExponentialOut,
                        EasingType::Once {
                            duration: std::time::Duration::from_secs(1),
                        },
                    )
                );
            });

            commands.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                DespawnTimer::seconds(1.),
            ))
            .insert(Transform::from_translation(vec3(20., 10., 8.) + start))
            .with_children(|commands| {
                commands.spawn((
                    Name::new("Particle"),
                    emotion_bundle(&asset_server, &mut layout_handles, rand::thread_rng().gen_range(0..3) + 6),
                    Transform::from_translation(vec3(0., 0., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(Vec3::splat(0.5))
                        .ease_to(
                            Transform::from_translation(vec3(rand::thread_rng().gen::<f32>() * 3. - 1.5, 4. + rand::thread_rng().gen::<f32>() * 5., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)),
                            EaseFunction::ExponentialOut,
                            EasingType::Once {
                                duration: std::time::Duration::from_secs(1),
                            },
                        )
                )).insert(
                    Sprite{flip_x: flipped, ..default()}.ease_to(
                        Sprite { color: Color::Srgba(Srgba::new(1., 1., 1., 0.)), flip_x: flipped,..default() },
                        EaseFunction::ExponentialOut,
                        EasingType::Once {
                            duration: std::time::Duration::from_secs(1),
                        },
                    )
                );
            });

        }
        if keyboard.pressed(KeyCode::AltLeft){
            let max_offset = 10.;
            let start = vec3(
                rand::random::<f32>() * max_offset * 2. - max_offset,
                rand::random::<f32>() * max_offset * 2. - max_offset,
                0.
            );

            let flipped = rand::thread_rng().gen_bool(0.5);
            commands.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                DespawnTimer::seconds(1.),
            ))
            .insert(Transform::from_translation(vec3(0., 10., 8.) + start))
            .with_children(|commands| {
                commands.spawn((
                    Name::new("Particle"),
                    SpriteBundle{
                        texture: asset_server.load("particles/body_civilian.png"),
                        ..default()
                    },
                    Transform::from_translation(vec3(0., 0., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(Vec3::splat(0.5))
                        .ease_to(
                            Transform::from_translation(vec3(rand::thread_rng().gen::<f32>() * 3. - 1.5, 4. + rand::thread_rng().gen::<f32>() * 5., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)),
                            EaseFunction::ExponentialOut,
                            EasingType::Once {
                                duration: std::time::Duration::from_secs(1),
                            },
                        )
                )).insert(
                    Sprite{flip_x: flipped, ..default()}.ease_to(
                        Sprite { color: Color::Srgba(Srgba::new(1., 1., 1., 0.)), flip_x: flipped,..default() },
                        EaseFunction::ExponentialOut,
                        EasingType::Once {
                            duration: std::time::Duration::from_secs(1),
                        },
                    )
                );
            });
        };

        if keyboard.just_pressed(KeyCode::Digit6){
            let max_offset = 20.;
            let start = vec3(
                rand::random::<f32>() * max_offset * 2. - max_offset,
                rand::random::<f32>() * max_offset * 2. - max_offset,
                0.
            );

            let flipped = rand::thread_rng().gen_bool(0.5);
            spawn_cililian_body(&mut commands, &mut layout_handles, &asset_server, vec3(10., 0., 0.));
        }

        if keyboard.just_pressed(KeyCode::Digit7){
            let max_offset = 20.;
            let start = vec3(
                rand::random::<f32>() * max_offset * 2. - max_offset,
                rand::random::<f32>() * max_offset * 2. - max_offset,
                0.
            );

            let flipped = rand::thread_rng().gen_bool(0.5);
            spawn_cililian_body(&mut commands, &mut layout_handles, &asset_server, vec3(10., 0., 0.));
        }
    }
}