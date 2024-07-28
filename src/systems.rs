use std::time::Duration;

use bevy::{math::uvec2, prelude::*};
use bevy_light_2d::light::AmbientLight2d;
use pathfinding::num_traits::{Euclid, Signed};

use crate::{characters::animation::AnimationController, core::{camera::plugin::MainCamera, functions::TextureAtlasLayoutHandles, post_processing::PostProcessUniform}, player::components::Player};

pub const TRANSLATION_DURATION: f32 = 1.0;
pub const DAY_DURATION: f32 = 1.0;

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
    mut cam: Query<&mut AmbientLight2d, With<MainCamera>>,
    time: Res<Time<Virtual>>,
    mut pc_q: Query<&mut AnimationController, With<Player>>
) {
    let cycle_time = (time.elapsed_seconds() + TRANSLATION_DURATION * 2. + DAY_DURATION * 2.) % (TRANSLATION_DURATION * 2. + DAY_DURATION * 2.);
    let is_night_raw = cycle_time < (TRANSLATION_DURATION + DAY_DURATION);
    let local_time = cycle_time % (TRANSLATION_DURATION + DAY_DURATION);
    cycle.is_night = is_night_raw;
    let mut light = cam.single_mut();
    cycle.is_translating = false;
    if local_time > DAY_DURATION {
        let translation = (local_time - DAY_DURATION) / TRANSLATION_DURATION;
        let v = if is_night_raw {1.-translation} else {translation};
        post_process.single_mut().daytime = v;
        light.brightness = (1. - v) * 0.8 + 0.2;
        cycle.is_translating = true;
        if translation > 0.5 {
            cycle.is_night = !cycle.is_night;
        }
    } else {
        post_process.single_mut().daytime = if cycle.is_night {1.} else {0.};
        light.brightness = if cycle.is_night {0.2} else {1.};
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
    mut time: ResMut<Time<Virtual>>
) {
    if keyboard.just_released(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => {
                next_state.set(GameState::Pause);
                time.pause();
            },
            GameState::Pause => {
                next_state.set(GameState::InGame);
                time.unpause();
            },
        }
    }
}