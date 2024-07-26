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

/*
Angry,
Warn,
Qestion,
*/

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



pub fn animated_heart_gain_bundle() -> impl Bundle {
    (
        SimpleAnimated::HeartGain
    )
}

pub fn animated_heart_loss_bundle() -> impl Bundle {
    (
        SimpleAnimated::HeartLoss
    )
}

pub fn animated_fork_bundle() -> impl Bundle {
    (
        SimpleAnimated::Fork
    )
}

pub fn animated_garlic_bundle() -> impl Bundle {
    (
        SimpleAnimated::Fork
    )
}

pub fn animated_soul_bundle() -> impl Bundle {
    (
        SimpleAnimated::Soul
    )
}




