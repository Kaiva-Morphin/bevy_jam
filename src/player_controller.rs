mod core;
mod map;


use core::camera::plugin::CameraFollow;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::{ScreenDiagnostics, ScreenDiagnosticsPlugin};
use core::default;

use bevy::input::keyboard::KeyboardInput;
use bevy::math::{uvec2, vec2};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::control::KinematicCharacterController;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use map::plugin::TileMapPlugin;
use pathfinding::num_traits::Signed;

use bevy_hanabi::prelude::*;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        TileMapPlugin
    ));
    app.add_plugins(HanabiPlugin);
    app.add_systems(Startup, setup);
    app.add_systems(Update, update);
    app.run();
}

#[derive(Component)]
pub struct PlayerController{
    accumulated_velocity: Vec2,
}
impl Default for PlayerController {
    fn default() -> Self {
        PlayerController{accumulated_velocity: Vec2::ZERO}
    }
}

#[derive(Default, Debug)]
enum Direction {
    Up,
    Right,
    #[default]
    Down,
    Left
}

#[derive(Component, Default)]
struct PlayerAnimationState{
    pub dir: Direction
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
){
    commands.spawn((
        VisibilityBundle::default(),
        TransformBundle::default(),
        Name::new("Player"),
        CameraFollow{order: 0, speed: 10_000.},
        RigidBody::KinematicPositionBased,
        Collider::ball(5.),
        KinematicCharacterController::default(),
        PlayerController::default(),
    )).with_children(|commands| {commands.spawn((
        SpriteBundle{
            texture: asset_server.load("vampire.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(14, 18), 3, 4, None, None)),
            index: 2
        },
        PlayerAnimationState::default()
    ));});
}



struct SpeedCFG{
    max_speed : f32,
    accumulation_grain : f32,
    follow_speed: f32
}

impl Default for SpeedCFG {
    fn default() -> Self {
        SpeedCFG {
            max_speed: 80.,
            accumulation_grain: 600.,
            follow_speed: 10.
        }
    }
}

fn update(
    mut player_q: Query<(&mut KinematicCharacterController, &mut PlayerController, &mut CameraFollow, &Transform), Without<PlayerAnimationState>>,
    mut player_sprite_q: Query<(&mut TextureAtlas, &mut Transform, &mut PlayerAnimationState), Without<PlayerController>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut speed_cfg: Local<SpeedCFG>,
    mut egui_context: EguiContexts,
    mut commands: Commands,
    asset_server: Res<AssetServer>
){  
    let ctx = egui_context.ctx_mut();

    egui::Window::new("SLIDERS").show(ctx, |ui|{
        ui.add(Slider::new(&mut speed_cfg.max_speed, 1. ..= 10_000.).text("MAX SPEED"));
        ui.add(Slider::new(&mut speed_cfg.accumulation_grain, 1. ..= 10_000.).text("ACCUMULATION GRAIN"));
        ui.add(Slider::new(&mut speed_cfg.follow_speed, 1. ..= 10_000.).text("CAMERA FOLLOW SPEED"));
    });

    let (mut character_controller, mut controller, mut follow, player_transform) = player_q.single_mut();

    follow.speed = speed_cfg.follow_speed;

    let input_dir = vec2(
        keyboard.pressed(KeyCode::KeyD) as i32 as f32 - keyboard.pressed(KeyCode::KeyA) as i32 as f32,
        keyboard.pressed(KeyCode::KeyW) as i32 as f32 - keyboard.pressed(KeyCode::KeyS) as i32 as f32
    );

    controller.accumulated_velocity = controller.accumulated_velocity.move_towards(input_dir.normalize_or_zero() * speed_cfg.max_speed, time.delta_seconds() * speed_cfg.accumulation_grain);
    if controller.accumulated_velocity.length() > speed_cfg.max_speed {controller.accumulated_velocity = controller.accumulated_velocity.normalize() * speed_cfg.max_speed}
    character_controller.translation = Some(controller.accumulated_velocity * time.delta_seconds());

    let (mut layout, mut transform, mut anim) = player_sprite_q.single_mut();
    
    if input_dir.x.abs() < 0.1 { // x axis is priotirized 
        if input_dir.y.abs() > 0.1 {
            if input_dir.y.is_positive(){anim.dir = Direction::Up}
            if input_dir.y.is_negative(){anim.dir = Direction::Down}
        }
    } else {
        if input_dir.x.is_positive(){anim.dir = Direction::Right}
        if input_dir.x.is_negative(){anim.dir = Direction::Left}
    }

    let index = match anim.dir {
        Direction::Up => {6},
        Direction::Right => {9},
        Direction::Down => {0},
        Direction::Left => {3},
    };

    transform.translation.y = ((time.elapsed_seconds() * 5.) as i32 % 2) as f32 * 0.5;
    if controller.accumulated_velocity.length() > 0.1 {
        let mut anim_offset = ((time.elapsed_seconds() * 5.) as i32 % 4) as usize;
        if anim_offset == 3 {anim_offset = 1} // wrap
        layout.index = index + anim_offset;
    } else {
        layout.index = index + 1;
    }



    let sprite_size = UVec2::new(48, 16);
    let sprite_grid_size = UVec2::new(3,  1);

    let texture_handle = asset_server.load("bat.png");

    // The sprites form a grid, with a total animation frame count equal to the
    // number of sprites.
    let frame_count = sprite_grid_size.x * sprite_grid_size.y;

    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::ONE);
    gradient.add_key(0.5, Vec4::ONE);
    gradient.add_key(1.0, Vec3::ONE.extend(0.));

    let writer = ExprWriter::new();

    let age = writer.rand(ScalarType::Float).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // All particles stay alive until their AGE is 5 seconds. Note that this doesn't
    // mean they live for 5 seconds; if the AGE is initialized to a non-zero value
    // at spawn, the total particle lifetime is (LIFETIME - AGE).
    let lifetime = writer.lit(5.).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::Y * 0.1).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(0.4).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Y).expr(),
        speed: (writer.lit(1.) + writer.lit(0.5) * writer.rand(ScalarType::Float)).expr(),
    };

    // Animate the SPRITE_INDEX attribute of each particle based on its age.
    // We want to animate back and forth the index in [0:N-1] where N is the total
    // number of sprites in the sprite sheet.
    // - For the back and forth, we build a linear ramp z 1 -> 0 -> 1 with abs(x)
    //   and y linear in [-1:1]
    // - To get that linear cyclic y variable in [-1:1], we build a linear cyclic x
    //   variable in [0:1]
    // - To get that linear cyclic x variable in [0:1], we take the fractional part
    //   of the age
    // - Because we want to have one full cycle every couple of seconds, we need to
    //   scale down the age value (0.02)
    // - Finally the linear ramp z is scaled to the [0:N-1] range
    // Putting it together we get:
    //   sprite_index = i32(
    //       abs(fract(particle.age * 0.02) * 2. - 1.) * frame_count
    //     ) % frame_count;
    let sprite_index = writer
        .attr(Attribute::AGE)
        .mul(writer.lit(0.1))
        .fract()
        .mul(writer.lit(2.))
        .sub(writer.lit(1.))
        .abs()
        .mul(writer.lit(frame_count as f32))
        .cast(ScalarType::Int)
        .rem(writer.lit(frame_count as i32))
        .expr();
    let update_sprite_index = SetAttributeModifier::new(Attribute::SPRITE_INDEX, sprite_index);

    let effect = asset_server.add(
        EffectAsset::new(
            vec![300],
            Spawner::burst(32.0.into(), 8.0.into()),
            writer.finish(),
        )
        .with_name("circle")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .update(update_sprite_index)
        .render(ParticleTextureModifier {
            texture: texture_handle.clone(),
            sample_mapping: ImageSampleMapping::Modulate,
        })
        .render(FlipbookModifier { sprite_grid_size })
        .render(ColorOverLifetimeModifier { gradient })
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant([0.5; 2].into()),
            screen_space_size: false,
        }),
    );



    if keyboard.just_pressed(KeyCode::ShiftLeft){
        commands
        .spawn(ParticleEffectBundle::new(effect))
        .insert(Name::new("effect"));
    }

}
