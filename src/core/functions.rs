use bevy::prelude::*;

pub trait ExpDecay<T> {
    fn exp_decay(&self, b: T, decay: f32, dt: f32) -> T;
}

impl ExpDecay<f32> for f32 {
    fn exp_decay(&self, b: f32, decay: f32, dt: f32) -> f32 {
        b + (self - b) * (-decay*dt).exp()
    }
}

impl ExpDecay<Vec3> for Vec3 {
    fn exp_decay(&self, b: Vec3, decay: f32, dt: f32) -> Vec3 {
        b + (*self - b) * (-decay*dt).exp()
    }
}