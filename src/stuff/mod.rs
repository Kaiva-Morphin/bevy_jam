use bevy::{math::uvec2, prelude::*};

#[derive(Component)]
pub enum SimpleAnimated{
    HeartGain,
    HeartLoss,
    Soul,
    Fork,
    Knife,
    Garlic
}

pub fn emotion_bundle(asset_server: &AssetServer, idx: usize) -> impl Bundle {
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

pub fn animated_heart_gain_bundle(asset_server: &AssetServer) -> impl Bundle {
    (
        SimpleAnimated::HeartGain,
        SpriteBundle{
            texture: asset_server.load("particles/emotions.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(9, 13), 3, 3, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}

pub fn animated_heart_loss_bundle(asset_server: &AssetServer,) -> impl Bundle {
    (
        SimpleAnimated::HeartLoss,
        SpriteBundle{
            texture: asset_server.load("particles/emotions.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(9, 13), 3, 3, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}

pub fn animated_fork_bundle(asset_server: &AssetServer,) -> impl Bundle {
    (
        SimpleAnimated::Fork,
        SpriteBundle{
            texture: asset_server.load("particles/emotions.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(9, 13), 3, 3, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}

pub fn animated_garlic_bundle(asset_server: &AssetServer,) -> impl Bundle {
    (
        SimpleAnimated::Garlic,
        SpriteBundle{
            texture: asset_server.load("particles/emotions.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(9, 13), 3, 3, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}


pub fn animated_soul_bundle(asset_server: &AssetServer,) -> impl Bundle {
    (
        SimpleAnimated::Soul,
        SpriteBundle{
            texture: asset_server.load("particles/emotions.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(9, 13), 3, 3, Some(uvec2(1, 1)), None)),
            index: 0
        },
    )
}




