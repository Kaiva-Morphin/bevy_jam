use std::{f32::consts::PI, time::Duration};

use bevy::{math::{uvec2, vec3}, prelude::*};
use bevy_easings::*;
use bevy_rapier2d::prelude::Velocity;
use rand::Rng;

use crate::core::{despawn_lifetime::DespawnTimer, functions::TextureAtlasLayoutHandles};

pub enum SimpleAnimatedTexture{
    HeartGain,
    HeartLoss,
    Soul,
    Fork,
    Knife,
    Garlic
}

#[derive(Component)]
pub struct SimpleAnimated{
    effect: SimpleAnimatedTexture,
    timer: Timer
}

pub fn simple_anim_update(
    mut to_anim: Query<(&mut SimpleAnimated, &mut TextureAtlas)>,
    time: Res<Time>
){
    let dt = time.delta_seconds();
    for (mut anim_type, mut atlas) in to_anim.iter_mut(){
        anim_type.timer.tick(Duration::from_secs_f32(dt.into()));

        let frames = match anim_type.effect {
            SimpleAnimatedTexture::HeartGain => 2,
            SimpleAnimatedTexture::HeartLoss => 2,
            SimpleAnimatedTexture::Soul => 5,
            SimpleAnimatedTexture::Fork => 8,
            SimpleAnimatedTexture::Knife => 8,
            SimpleAnimatedTexture::Garlic => 4,
        };
        let offset = match anim_type.effect {
            SimpleAnimatedTexture::HeartGain => 2,
            SimpleAnimatedTexture::HeartLoss => 0,
            SimpleAnimatedTexture::Soul => 0,
            SimpleAnimatedTexture::Fork => 10,
            SimpleAnimatedTexture::Knife => 0,
            SimpleAnimatedTexture::Garlic => 30,
        };

        if anim_type.timer.finished(){
            atlas.index = (atlas.index + 1 - offset) % frames + offset
        }
    }
}

pub fn emotion_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>, idx: usize) -> impl Bundle {
    (
        SpriteBundle{
            texture: asset_server.load("particles/emotions.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(9, 13), 3, 3, Some(uvec2(1, 1)), None)),
            index: idx
        },
    )
}

pub fn animated_heart_gain_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>, ) -> impl Bundle {
    (
        SimpleAnimated{effect: SimpleAnimatedTexture::HeartGain, timer: Timer::from_seconds(0.06, TimerMode::Repeating)},
        SpriteBundle{
            texture: asset_server.load("particles/heart.png"),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Hearts", TextureAtlasLayout::from_grid(uvec2(10, 9), 2, 2, Some(uvec2(1, 1)), None)),
            index: 2
        },
    )
}

pub fn animated_heart_loss_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>,) -> impl Bundle {
    (
        SimpleAnimated{effect: SimpleAnimatedTexture::HeartLoss, timer: Timer::from_seconds(0.06, TimerMode::Repeating)},
        SpriteBundle{
            texture: asset_server.load("particles/heart.png"),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Hearts", TextureAtlasLayout::from_grid(uvec2(10, 9), 2, 2, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}

pub fn animated_fork_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>,) -> impl Bundle {
    (
        SimpleAnimated{effect: SimpleAnimatedTexture::Fork, timer: Timer::from_seconds(0.06, TimerMode::Repeating)},
        SpriteBundle{
            texture: asset_server.load("hunter/throwables.png"),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Throwables", TextureAtlasLayout::from_grid(uvec2(13, 13), 10, 4, Some(uvec2(1, 1)), None)),
            index: 10
        },
    )
}

pub fn animated_garlic_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>,) -> impl Bundle {
    (
        SimpleAnimated{effect: SimpleAnimatedTexture::Garlic, timer: Timer::from_seconds(0.06, TimerMode::Repeating)},
        SpriteBundle{
            texture: asset_server.load("hunter/throwables.png"),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Throwables", TextureAtlasLayout::from_grid(uvec2(13, 13), 10, 4, Some(uvec2(1, 1)), None)),
            index: 30
        },
    )
}

pub fn animated_knife_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>,) -> impl Bundle {
    (
        SimpleAnimated{effect: SimpleAnimatedTexture::Knife, timer: Timer::from_seconds(0.06, TimerMode::Repeating)},
        SpriteBundle{
            texture: asset_server.load("hunter/throwables.png"),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Throwables", TextureAtlasLayout::from_grid(uvec2(13, 13), 10, 4, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}

pub fn animated_soul_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>,) -> impl Bundle {
    (
        SimpleAnimated{effect: SimpleAnimatedTexture::Soul, timer: Timer::from_seconds(0.1, TimerMode::Repeating)},
        DespawnTimer::seconds(0.5),
        SpriteBundle{
            texture: asset_server.load("particles/soul.png"),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Soul", TextureAtlasLayout::from_grid(uvec2(9, 12), 5, 1, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}

#[derive(Component)]
pub struct Stake;

pub fn stake_bundle(asset_server: &Res<AssetServer>, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>, direction: Vec2) -> impl Bundle {
    let angle = direction.to_angle() + PI * 0.75;
    (
        SpriteBundle{
            texture: asset_server.load("hunter/throwables.png"),
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, angle, 0., 0.)),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Throwables", TextureAtlasLayout::from_grid(uvec2(13, 13), 10, 4, Some(uvec2(1, 1)), None)),
            index: 20
        },
    )
}




pub fn spawn_cililian_body(
    commands: &mut Commands,
    layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
) -> Entity {

    let max_offset = 4.;
    let start = pos + vec3(
        rand::random::<f32>() * max_offset * 2. - max_offset,
        rand::random::<f32>() * max_offset * 2. - max_offset,
        0.
    );
    let flipped = rand::thread_rng().gen_bool(0.5);
    let offset = if flipped{vec3(-2., 0., 0.)} else {vec3(2., 0., 0.)};
    commands.spawn(animated_soul_bundle(asset_server, layout_handles))
    .insert(Transform::from_translation(offset+vec3(0., 8., 10.) + start).ease_to(
        Transform::from_translation(offset+start+vec3(0., 12. + rand::thread_rng().gen::<f32>() * 5., 10.)),
        EaseFunction::ExponentialOut,
        EasingType::Once {
            duration: std::time::Duration::from_secs(1),
        },
    ));

    commands.spawn((
        TransformBundle::default(),
        VisibilityBundle::default(),
        DespawnTimer::seconds(5.),
    ))
    .insert(Transform::from_translation(vec3(0., 0., 8.) + start).with_scale(vec3(if flipped{-1.} else {1.}, 1., 1.)))
    .with_children(|commands| {
        commands.spawn((
            Name::new("Particle"),
            SpriteBundle{
                texture: asset_server.load("particles/body_civilian.png"),
                ..default()
            },
        )).insert(
            Sprite{..default()}.ease_to(
                Sprite { color: Color::Srgba(Srgba::new(1., 1., 1., 0.)), ..default() },
                EaseFunction::ExponentialIn,
                EasingType::Once {
                    duration: std::time::Duration::from_secs(5),
                },
            )
        );
    }).id()
}

pub fn spawn_hunter_body(
    commands: &mut Commands,
    layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
) -> Entity {
    let max_offset = 4.;
    let start = pos + vec3(
        rand::random::<f32>() * max_offset * 2. - max_offset,
        rand::random::<f32>() * max_offset * 2. - max_offset,
        0.
    );
    let flipped = rand::thread_rng().gen_bool(0.5);
    let offset = if flipped{vec3(-2., 0., 0.)} else {vec3(2., 0., 0.)};
    commands.spawn(animated_soul_bundle(asset_server, layout_handles))
    .insert(Transform::from_translation(offset+vec3(0., 8., 10.) + start).ease_to(
        Transform::from_translation(offset+start+vec3(0., 12. + rand::thread_rng().gen::<f32>() * 5., 10.)),
        EaseFunction::ExponentialOut,
        EasingType::Once {
            duration: std::time::Duration::from_secs(1),
        },
    ));

    commands.spawn((
        TransformBundle::default(),
        VisibilityBundle::default(),
        DespawnTimer::seconds(5.),
    ))
    .insert(Transform::from_translation(vec3(0., 0., 8.) + start).with_scale(vec3(if flipped{-1.} else {1.}, 1., 1.)))
    .with_children(|commands| {
        commands.spawn((
            Name::new("Particle"),
            SpriteBundle{
                texture: asset_server.load("particles/body_hunter.png"),
                ..default()
            },
        )).insert(
            Sprite{..default()}.ease_to(
                Sprite { color: Color::Srgba(Srgba::new(1., 1., 1., 0.)), ..default() },
                EaseFunction::ExponentialIn,
                EasingType::Once {
                    duration: std::time::Duration::from_secs(5),
                },
            )
        );
    }).id()
}

pub fn spawn_question_particle(
    commands: &mut Commands,
    layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
){
    let max_offset = 7.;
    let start = pos+vec3(
        rand::random::<f32>() * max_offset * 2. - max_offset,
        rand::random::<f32>() * max_offset * 2. - max_offset,
        0.
    );
    commands.spawn((
        TransformBundle::default(),
        VisibilityBundle::default(),
        DespawnTimer::seconds(1.),
    ))
    .insert(Transform::from_translation(vec3(20., 10., 8.) + start))
    .with_children(|commands| {
        commands.spawn((
            Name::new("Particle"),
            emotion_bundle(asset_server, layout_handles, rand::thread_rng().gen_range(0..3) + 6),
            Transform::from_translation(vec3(0., 0., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(Vec3::splat(0.5))
                .ease_to(
                    Transform::from_translation(vec3(rand::thread_rng().gen::<f32>() * 3. - 1.5, 4. + rand::thread_rng().gen::<f32>() * 5., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)),
                    EaseFunction::ExponentialOut,
                    EasingType::Once {
                        duration: std::time::Duration::from_secs(1),
                    },
                )
        )).insert(
            Sprite{..default()}.ease_to(
                Sprite { color: Color::Srgba(Srgba::new(1., 1., 1., 0.)),..default() },
                EaseFunction::ExponentialIn,
                EasingType::Once {
                    duration: std::time::Duration::from_secs(1),
                },
            )
        );
    });
}

pub fn spawn_angry_particle(
    commands: &mut Commands,
    layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
){
    let max_offset = 7.;
    let start = pos+vec3(
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
            emotion_bundle(asset_server, layout_handles, rand::thread_rng().gen_range(0..3)),
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
}

pub fn spawn_warn_particle(
    commands: &mut Commands,
    layout_handles: &mut ResMut<TextureAtlasLayoutHandles>,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
){
    let max_offset = 7.;
    let start = pos+vec3(
        rand::random::<f32>() * max_offset * 2. - max_offset,
        rand::random::<f32>() * max_offset * 2. - max_offset,
        0.
    );
    commands.spawn((
        TransformBundle::default(),
        VisibilityBundle::default(),
        DespawnTimer::seconds(1.),
    ))
    .insert(Transform::from_translation(vec3(10., 20., 8.) + start))
    .with_children(|commands| {
        commands.spawn((
            Name::new("Particle"),
            emotion_bundle(asset_server, layout_handles, rand::thread_rng().gen_range(0..3) + 3),
            Transform::from_translation(vec3(0., 0., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)).with_scale(Vec3::splat(0.5))
                .ease_to(
                    Transform::from_translation(vec3(rand::thread_rng().gen::<f32>() * 3. - 1.5, 4. + rand::thread_rng().gen::<f32>() * 5., 0.)).with_rotation(Quat::from_rotation_z(rand::thread_rng().gen::<f32>() - 0.5)),
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
}

