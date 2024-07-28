use std::time::Duration;

use bevy::{math::uvec2, prelude::*};
use pathfinding::num_traits::{Euclid, Signed};

use crate::{characters::animation::AnimationController, core::{functions::TextureAtlasLayoutHandles, post_processing::PostProcessUniform}, player::components::Player};

pub const TRANSLATION_DURATION: f32 = 1.0;
pub const DAY_DURATION: f32 = 10.0;

#[derive(Resource)]
pub struct DayCycle {
    pub is_night: bool,
    pub is_translating: bool,
}

// 0 is morning
pub fn get_local_time_f(elapsed: f32) -> f32{
    ((elapsed + TRANSLATION_DURATION * 0.5) % (TRANSLATION_DURATION * 2. + DAY_DURATION * 2.)) / (TRANSLATION_DURATION * 2. + DAY_DURATION * 2.)
}


pub fn update_daycycle(
    mut cycle: ResMut<DayCycle>,
    mut post_process: Query<&mut PostProcessUniform>,
    time: Res<Time<Virtual>>,
    mut pc_q: Query<&mut AnimationController, With<Player>>
) {
    let cycle_time = (time.elapsed_seconds() + TRANSLATION_DURATION * 2. + DAY_DURATION * 2.) % (TRANSLATION_DURATION * 2. + DAY_DURATION * 2.);
    let is_night_raw = cycle_time > (TRANSLATION_DURATION + DAY_DURATION);
    let local_time = cycle_time % (TRANSLATION_DURATION + DAY_DURATION);
    cycle.is_night = is_night_raw;
    if local_time > DAY_DURATION {
        let translation = (local_time - DAY_DURATION) / TRANSLATION_DURATION;
        post_process.single_mut().daytime = if is_night_raw {translation} else {1.-translation};
        if translation > 0.5 {
            cycle.is_night = !cycle.is_night;
        }
    } else {
        post_process.single_mut().daytime = if cycle.is_night {0.} else {1.}
    }
    for mut pc in pc_q.iter_mut(){
        if cycle.is_night {
            pc.disarm();
        } else {
            pc.arm();
        }
    }
}

fn f(x: f32) -> f32 {
    if x < 0.5 {
        return 2. * x * x;
    } else {
        return 1. - (-2. * x + 2.).powf(2.) / 2.;
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    InGame,
    Pause,
}

pub fn pause_game(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: ResMut<Time<Virtual>>
) {
    if keyboard.just_released(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Pause),
            GameState::Pause => next_state.set(GameState::InGame),
        }
    }
}