


pub mod plugin {
    use bevy::{app::{Plugin, Startup, Update}, color::{Color, Srgba}, core_pipeline::tonemapping::{DebandDither, Tonemapping}, math::vec2, prelude::{default, App, Camera2d, Camera2dBundle, Commands, Gizmos, PluginGroup, Transform, Vec3}, render::{camera::{Camera, OrthographicProjection, ScalingMode}, texture::ImagePlugin, view::Msaa}, window::{PresentMode, Window, WindowPlugin, WindowTheme}, DefaultPlugins};
    use bevy_rapier2d::render::RapierDebugRenderPlugin;
    use bevy_rapier2d::prelude::*;

    use crate::core::{camera::plugin::EnhancedCameraPlugin, debug::rapier_debug::plugin::SwitchableRapierDebugPlugin, post_processing::PostProcessPlugin};
    pub struct DefaultPlugin;

    impl Plugin for DefaultPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                                present_mode: PresentMode::AutoNoVsync,
                                window_theme: Some(WindowTheme::Dark),
                                title: "Bloody Night".into(),
                                ..default()
                            }),
                            ..default()
                        }),
                RapierDebugRenderPlugin::default().disabled(),
                RapierPhysicsPlugin::<NoUserData>::default(),
                SwitchableRapierDebugPlugin,
                EnhancedCameraPlugin,
                PostProcessPlugin
            ),
            );
            app.insert_resource(Msaa::Off);
            app.insert_resource(RapierConfiguration {
                gravity: vec2(0.0, 0.0),
                physics_pipeline_active: true,
                query_pipeline_active: true,
                timestep_mode: TimestepMode::Variable {
                    max_dt: 1.0 / 60.0,
                    time_scale: 1.0,
                    substeps: 1,
                },
                scaled_shape_subdivision: 10,
                force_update_from_transform_changes: false,
            });
            app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
        }
    }
}

