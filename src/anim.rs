pub mod core;
pub mod player;
pub mod npc;
pub mod map;
pub mod systems;
pub mod characters;
pub mod stuff;

use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::ScreenDiagnosticsPlugin;
use core::despawn_lifetime::DespawnTimer;
use core::functions::TextureAtlasLayoutHandles;
use crate::characters::animation::CivilianAnims;
use bevy::math::vec3;
use bevy::prelude::*;

use bevy_easings::{Ease, EaseFunction, EasingType};
use characters::animation::{spawn_civilian_animation_bundle, spawn_player_animation_bundle, AnimationController};
use characters::plugin::CharacterAnimationPlugin;
use stuff::emotion_bundle;

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
    app.add_systems(PreUpdate, update);
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
    let entity = spawn_civilian_animation_bundle(&mut commands, &asset_server, &mut layout_handles);
    commands.entity(entity).insert(PreviewCharacter);
}



fn update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut controller: Query<(&mut AnimationController, Entity), With<PreviewCharacter>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layout_handles: ResMut<TextureAtlasLayoutHandles>
){
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
            controller.play_walk();
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
                rand::random::<f32>() * max_offset - max_offset * 2.,
                rand::random::<f32>() * max_offset - max_offset * 2.,
                0.
            );
            commands.spawn((
                TransformBundle::default(),
                VisibilityBundle::default(),
                DespawnTimer::seconds(1.),
            ))
            .insert(Transform::from_translation(vec3(20., 0., 0.) + start))
            .with_children(|commands| {
                commands.spawn((
                    Name::new("Particle"),
                    emotion_bundle(&asset_server, 0),
                    Transform::from_translation(vec3(0., 0., 0.))
                        .ease_to(
                            Transform::from_translation(vec3(0., 5., 0.)),
                            EaseFunction::ExponentialOut,
                            EasingType::Once {
                                duration: std::time::Duration::from_secs(1),
                            },
                        )
                ));
            });
        }
        
    }
}