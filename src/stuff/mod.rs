use std::{f32::consts::PI, time::Duration};

use bevy::{math::uvec2, prelude::*};
use bevy_rapier2d::prelude::Velocity;

use crate::core::functions::TextureAtlasLayoutHandles;

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

/*pub fn animated_soul_bundle(asset_server: &AssetServer, atlas_handles: &mut ResMut<TextureAtlasLayoutHandles>,) -> impl Bundle {
    (
        SimpleAnimated{effect: SimpleAnimatedTexture::Soul, timer: Timer::from_seconds(0.06, TimerMode::Repeating)},
        SpriteBundle{
            texture: asset_server.load("hunter/throwables.png"),
            ..default()
        },
        TextureAtlas{
            layout: atlas_handles.add_or_load(asset_server, "Throwables", TextureAtlasLayout::from_grid(uvec2(13, 13), 10, 4, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}*/

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





