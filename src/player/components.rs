use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub hp: f32,
    pub xp: f32,
    pub score: f32,
    pub max_speed: f32,
    pub accumulation_grain: f32,
    pub phys_res: f32,
    pub hp_gain: f32,
    pub xp_gain: f32,
    pub max_hp: f32,
} 
// armor (phys res); speed; hp gain; xp gain; max hp;

#[derive(Component)]
pub struct DashTimer {
    pub timer: Timer,
}

#[derive(Event)]
pub struct HitPlayer {
    pub dmg_type: u8,
}

#[derive(Event)]
pub struct KillNpc {
    pub npc_type: u8,
}