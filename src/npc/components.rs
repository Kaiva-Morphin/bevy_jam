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

#[derive(Component)]
pub struct NpcVelAccum {
    pub v: Vec2,
}

#[derive(Component)]
pub struct NpcPath {
    pub path: Option<Vec<IVec2>>,
}

#[derive(Component, Clone, Copy)]
pub enum NpcState {
    Attack,
    Escape,
    Chase,
    Chill,
    Dead,
}

#[derive(Resource, Default)]
pub struct NpcsOnMap {
    pub hunters: u32,
    pub civilians: u32,
}