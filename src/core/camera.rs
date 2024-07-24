pub mod plugin{

use std::{cell::RefCell, default, sync::Mutex};

use bevy::{core_pipeline::{bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings}, tonemapping::{DebandDither, Tonemapping}}, input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::egui::mutex::RwLock;

use crate::core::functions::ExpDecay;
pub struct EnhancedCameraPlugin;



impl Plugin for EnhancedCameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_camera);
        app.add_systems(PostUpdate, update_camera);
    }
}


#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Default)]
pub struct CameraFollow{
    pub speed: f32,
    pub order: u32
}




#[derive(Component, Default)]
pub struct CameraController{
    scale: f32,
    scale_translation_speed: Option<f32>,
}



impl CameraController{
    fn set_scale(&mut self, new_scale: f32, translation_speed: Option<f32>){
        self.scale = new_scale;
        self.scale_translation_speed = translation_speed;
    }
}

fn setup_camera(
    mut commands: Commands,
){
    commands.spawn((
        Camera2dBundle{
            camera: Camera{
                hdr: true,
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: 400.0,
                    min_height: 300.0,
                },
                near: -800.,
                far: 800.,
                ..default()
            },
            tonemapping: Tonemapping::None,
            deband_dither: DebandDither::Disabled,
            transform: Transform::from_scale(Vec3::splat(1.)),
            ..default()
        },
        MainCamera,
        BloomSettings{
            intensity: 0.5,
            low_frequency_boost: 0.8,
            low_frequency_boost_curvature: 0.8,
            high_pass_frequency: 1.6,
            prefilter_settings: BloomPrefilterSettings{
                threshold: 0.6,
                threshold_softness: 0.7
            },
            composite_mode: BloomCompositeMode::Additive
        },
        CameraController{scale: 1., ..default()},
    ));
}

struct CameraScale(f32);

impl Default for CameraScale{
    fn default() -> Self {
        CameraScale(1.)
    }
}

fn update_camera(
    mut cameras_q: Query<(&mut Transform, &mut CameraController)>,
    targets_q: Query<(&Transform, &CameraFollow), (With<CameraFollow>, Without<CameraController>)>,
    time: Res<Time>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut camera_scale: Local<CameraScale>
){
    let mut follow_position = Vec3::ZERO;
    let mut highest = 0;
    let mut follow_speed = 0.;
    for (tansform, follow) in targets_q.iter(){
        if highest <= follow.order {
            follow_position = tansform.translation;
            highest = follow.order;
            follow_speed = follow.speed;
        }
    }
    

    for ev in evr_scroll.read() {
        camera_scale.0 -= ev.y * 0.1;
        camera_scale.0 = camera_scale.0.clamp(0.5, 3.);
    }

    for (mut camera_transform, mut controller) in cameras_q.iter_mut(){
        controller.scale = camera_scale.0 * camera_scale.0;
        controller.scale_translation_speed = Some(10.);

        camera_transform.translation = camera_transform.translation.exp_decay(follow_position, follow_speed, time.delta_seconds());
        let Some(scale_speed) = controller.scale_translation_speed else {camera_transform.scale = Vec2::splat(controller.scale).extend(1.);continue};
        camera_transform.scale = Vec2::splat(camera_transform.scale.x.exp_decay(controller.scale, scale_speed, time.delta_seconds())).extend(1.);
    }
}

}
