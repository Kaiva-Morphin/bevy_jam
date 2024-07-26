use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub hp: i32,
    pub xp: i32,
    pub score: i32,
    pub max_speed: f32,
    pub accumulation_grain: f32,
} // armor (phys res); speed; hp gain; xp gain; max hp;
// todo: add get dmg fn -> play hurt anim -= hp

#[derive(Component)]
pub struct DashTimer {
    pub timer: Timer,
}