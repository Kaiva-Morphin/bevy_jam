use bevy::prelude::*;

#[derive(Resource)]
pub struct DayCycle {
    pub time: f32,
    pub is_day: bool,
}

pub fn update_daycycle(
    mut cycle: ResMut<DayCycle>,
    time: Res<Time>,
) {
    cycle.time += time.delta_seconds();
    let t = cycle.time;
    if t < 20. {
        cycle.is_day = true;
    } else if t < 40. {
        cycle.is_day = false;
    } else {
        cycle.time = 0.;
    }
}