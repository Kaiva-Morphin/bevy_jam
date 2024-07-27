use bevy::{math::uvec2, prelude::*};

use crate::core::{functions::TextureAtlasLayoutHandles, post_processing::PostProcessUniform};

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
    cycle.daytime = 0.;
    cycle.is_night = false;
    return;
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
) {
    if keyboard.just_released(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Pause),
            GameState::Pause => next_state.set(GameState::InGame),
        }
    }
}

pub fn spawn_ui(
    mut commands: Commands,
    mut layout_handles: ResMut<TextureAtlasLayoutHandles>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100. / 4.78),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                bottom: Val::Px(0.),
                ..default()
            },
            ..default()
        },
        Name::new("UI"),
    )).with_children(|parent| {
        let w = 70.;
        parent.spawn(ImageBundle {
            style: Style {
                width: Val::Percent(w),
                height: Val::Percent(w * 0.2 * 4.78),
                // right: Val::Percent(50.),                
                ..default()
            },
            image: UiImage::new(asset_server.load("ui/new_ui.png")),
            ..default()
        });
    });
}