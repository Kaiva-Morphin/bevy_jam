use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct Civilian;

#[derive(Component)]
pub struct Hunter;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct DespawnTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct HunterTimer {
    pub timer: Timer
}