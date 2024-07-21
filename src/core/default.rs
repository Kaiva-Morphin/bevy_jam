


pub const DEFAULT_FONT_BYTES: &'static [u8; 202764] = include_bytes!("../../assets/fonts/Monocraft.ttf");

pub mod plugin {
    use bevy::{app::{Plugin, Startup}, math::vec2, prelude::{default, App, Camera2dBundle, Commands, PluginGroup, Transform, Vec3}, render::texture::ImagePlugin, window::{PresentMode, Window, WindowPlugin}, DefaultPlugins};
    use bevy_rapier2d::render::RapierDebugRenderPlugin;
    use bevy_rapier2d::prelude::*;

    use crate::core::debug::rapier_debug::plugin::SwitchableRapierDebugPlugin;

    pub struct DefaultPlugin;

    impl Plugin for DefaultPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins((
                DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        title: "Simple Game!".into(),
                            ..default()
                        }),
                            ..default()
                        }),
                RapierDebugRenderPlugin::default().disabled(),
                RapierPhysicsPlugin::<NoUserData>::default(),
                SwitchableRapierDebugPlugin,
                ),
            );
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
            app.add_systems(Startup, setup);
        }
    }

    fn setup(
        mut commands: Commands,
    ){
        commands.spawn(Camera2dBundle{
            transform: Transform::from_scale(Vec3::splat(0.5)),
            ..default()
        });
    }
}

