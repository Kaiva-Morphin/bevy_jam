use bevy::prelude::*;

use crate::core::post_processing::PostProcessUniform;

pub const TRANSLATION_DURATION: f32 = 5.0;
pub const DAY_DURATION: f32 = 10.0;

#[derive(Resource)]
pub struct DayCycle {
    pub translation_timer: Timer,
    pub cycle_timer: Timer,
    pub daytime: f32,
    pub is_night: bool,
    pub is_translating: bool,
}

pub fn update_daycycle(
    mut cycle: ResMut<DayCycle>,
    mut post_process: Query<&mut PostProcessUniform>,
    time: Res<Time>,
) {
    let delta = time.delta();
    if cycle.is_translating {
        cycle.translation_timer.tick(delta);
        let mut elapsed = cycle.translation_timer.elapsed_secs(); 
        elapsed /= TRANSLATION_DURATION;
        
        if cycle.translation_timer.finished() {
            cycle.is_translating = false;
            cycle.is_night = !cycle.is_night;
            return;
        }
        
        if cycle.is_night {
            elapsed = 1. - elapsed;
        }
        
        cycle.daytime = f(elapsed);
        post_process.single_mut().daytime = cycle.daytime;
        // x < 0.5 ? 2 * x * x : 1 - Math.pow(-2 * x + 2, 2) / 2
    } else {
        cycle.cycle_timer.tick(delta);
        
        if cycle.cycle_timer.finished() {
            cycle.is_translating = true;
            return;
        }
        
        if cycle.is_night {
            post_process.single_mut().daytime = 1.;
        } else {
            post_process.single_mut().daytime = 0.;
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