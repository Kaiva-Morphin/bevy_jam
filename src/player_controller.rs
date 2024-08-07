mod core;
mod player;
mod npc;
mod map;
pub mod systems;
mod characters;
pub mod stuff;

use crate::characters::animation::*;
use core::camera::plugin::CameraFollow;
use core::debug::egui_inspector::plugin::SwitchableEguiInspectorPlugin;
use core::debug::diagnostics_screen::plugin::{ScreenDiagnostics, ScreenDiagnosticsPlugin};

use bevy::math::{ivec2, uvec2, vec2};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, Slider};
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use characters::animation::AnimationController;
use characters::plugin::CharacterAnimationPlugin;
use map::plugin::TileMapPlugin;
use map::tilemap::TransformToGrid;
use pathfinding::num_traits::Signed;

use bevy_hanabi::prelude::*;

fn main() {
    let mut app = App::new();
    app
    .add_plugins((
        core::default::plugin::DefaultPlugin,
        SwitchableEguiInspectorPlugin,
        ScreenDiagnosticsPlugin,
        CharacterAnimationPlugin,
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


#[derive(Component)]
struct BatEffect;

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
){
    commands.spawn((
        VisibilityBundle::default(),
        TransformBundle::default(),
        Name::new("Player"),
        CameraFollow{order: 0, speed: 10_000.},
        RigidBody::Dynamic,
        Collider::ball(5.),
        Velocity::default(),
        PlayerController::default(),
        AnimationController::default(),
    )).with_children(|commands| {commands.spawn((
        SpriteBundle{
            texture: asset_server.load("player/vampire.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(14, 20), 7, 3, Some(uvec2(1, 1)), None)),
            index: 2
        }
        /*SpriteBundle{
            texture: asset_server.load("hunter.png"),
            ..default()
        },
        TextureAtlas{
            layout: asset_server.add(TextureAtlasLayout::from_grid(uvec2(16, 20), 7, 3, Some(uvec2(1, 1)), None)),
            index: 2
        }*/
    ));});

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
        axis: writer.lit(Vec3::Z).expr(),
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
            gradient: Gradient::constant([10.; 2].into()),
            screen_space_size: false,
        }),
    );



    commands
        .spawn(ParticleEffectBundle::new(effect))
        .insert(Name::new("effect"))
        .insert(BatEffect);
}

use core::functions::ExpDecay;


struct SpeedCFG{
    max_speed : f32,
    accumulation_gain : f32,
    follow_speed: f32
}

impl Default for SpeedCFG {
    fn default() -> Self {
        SpeedCFG {
            max_speed: 80.,
            accumulation_gain: 600.,
            follow_speed: 10.
        }
    }
}

fn update(
    mut player_q: Query<(&mut Velocity, &mut PlayerController, &mut CameraFollow, &Transform)>,
    mut animation_controller: Query<&mut AnimationController>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut speed_cfg: Local<SpeedCFG>,
    mut egui_context: EguiContexts,
    mut accum: Local<f32>,
    mut dir: Local<Option<Vec2>>,
){  
    let ctx = egui_context.ctx_mut();
    egui::Window::new("SLIDERS").show(ctx, |ui|{
        ui.add(Slider::new(&mut speed_cfg.max_speed, 1. ..= 10_000.).text("MAX SPEED"));
        ui.add(Slider::new(&mut speed_cfg.accumulation_gain, 1. ..= 10_000.).text("ACCUMULATION GRAIN"));
        ui.add(Slider::new(&mut speed_cfg.follow_speed, 1. ..= 10_000.).text("CAMERA FOLLOW SPEED"));
    });

    let (mut character_controller, mut controller, mut follow, player_transform) = player_q.single_mut();
    let mut animation_controller = animation_controller.single_mut();


    follow.speed = speed_cfg.follow_speed;

    let input_dir = vec2(
        keyboard.pressed(KeyCode::KeyD) as i32 as f32 - keyboard.pressed(KeyCode::KeyA) as i32 as f32,
        keyboard.pressed(KeyCode::KeyW) as i32 as f32 - keyboard.pressed(KeyCode::KeyS) as i32 as f32
    );

    if keyboard.just_pressed(KeyCode::ShiftLeft){
        *accum = 10.;
        *dir = Some(vec2(
            keyboard.pressed(KeyCode::KeyD) as i32 as f32 - keyboard.pressed(KeyCode::KeyA) as i32 as f32,
            keyboard.pressed(KeyCode::KeyW) as i32 as f32 - keyboard.pressed(KeyCode::KeyS) as i32 as f32
        ));
    }
    //character_controller.linvel = Vec2::ZERO;
    if let Some(udir) = *dir {
        character_controller.linvel = *accum * udir * 100.;
        *accum = (*accum).exp_decay(0., 10., time.delta_seconds());
        if *accum < 1. {*dir = None}
    } else {
        controller.accumulated_velocity = controller.accumulated_velocity.move_towards(input_dir.normalize_or_zero() * speed_cfg.max_speed, time.delta_seconds() * speed_cfg.accumulation_gain);
        if controller.accumulated_velocity.length() > speed_cfg.max_speed {controller.accumulated_velocity = controller.accumulated_velocity.normalize() * speed_cfg.max_speed}
        character_controller.linvel = controller.accumulated_velocity;
    };
    

    //let (mut layout, mut transform, mut anim) = player_sprite_q.single_mut();
    
    if input_dir.x.abs() < 0.1 { // x axis is priotirized 
        if input_dir.y.abs() > 0.1 {
            if input_dir.y.is_positive(){animation_controller.turn_up()}
            if input_dir.y.is_negative(){animation_controller.turn_down()}
        }
    } else {
        if input_dir.x.is_positive(){animation_controller.turn_right()}
        if input_dir.x.is_negative(){animation_controller.turn_left()}
    }
    if controller.accumulated_velocity.length() > 0.1 {
        animation_controller.play_walk();
    } else {
        animation_controller.play_idle_priority(1);
    }
    if keyboard.just_pressed(KeyCode::Digit1){
        animation_controller.play_hurt();
    }
    if keyboard.just_pressed(KeyCode::Digit2){
        animation_controller.play_hunter_throw();
    }
    if keyboard.just_pressed(KeyCode::Digit3){
        animation_controller.play_hunter_throw();
    }
    /*transform.translation.y = ((time.elapsed_seconds() * 5.) as i32 % 2) as f32 * 0.5;
    if controller.accumulated_velocity.length() > 0.1 {
        let mut anim_offset = ((time.elapsed_seconds() * 5.) as i32 % 4) as usize;
        if anim_offset == 3 {anim_offset = 1} // wrap
        layout.index = index + anim_offset;
    } else {
        layout.index = index + 1;
    }*/
}
