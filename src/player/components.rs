use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub hp: i32,
    pub xp: i32,
    pub score: i32,
}