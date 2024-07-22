use bevy::prelude::*;

pub const PLAYER_CG: u32 = 0b0000_0000_0000_0001;
pub const NPC_CG: u32 = 0b0000_0000_0000_0010;
pub const STRUCTURES_CG: u32 = 0b0000_0000_0000_0100;
pub const BULLET_CG: u32 = 0b0000_0000_0000_1000;

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