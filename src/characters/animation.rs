use bevy::{color::palettes::css::RED, math::Direction2d, prelude::*};



#[derive(Component)]
pub struct AnimationController{
    animation_speed: f32,
    ticker: AnimTicker,
    current_animation: CharacterAnimation,
    dir_offset: usize,
    /*
    0 - idle, talk
    1 - move
    2 - attack, skill
    3 - hurt
    */
    priority: usize,
    direction: usize,
}


#[derive(Default)]
pub struct AnimTicker{
    frame: f32
}

impl AnimTicker{
    pub fn to_start(&mut self){
        self.frame = 0.;
    }
    pub fn tick(&mut self, dt: f32){
        self.frame += dt;
    }

    fn get<T: Clone>(&self, frametime: &FrameTime, vec: &Vec<T>) -> Option<T> {
        if vec.len() == 0 {return None}
        match frametime {
            FrameTime::Constant(t) => {
                let current_anim_frame = (self.frame / t).floor() as usize;
                if current_anim_frame >= vec.len() {return vec.last().cloned()}
                return vec.get(current_anim_frame).cloned()
            },
            FrameTime::Sequence(v) => {
                let mut ac = 0.;
                let mut lasti = 0;
                for (i, t) in v.iter().enumerate() {
                    if ac >= self.frame {
                        if i >= vec.len() {return vec.last().cloned()}
                        return vec.get(i.checked_sub(1).unwrap_or(0)).cloned()
                    }
                    ac += t;
                    lasti = i;
                }
                return vec.get(lasti).cloned()
            },
        }
    }
}


pub trait PlayerAnims{

}

pub trait HunterAnims{
    fn play_hunter_throw(&mut self){}
}

pub trait CivilianAnims{

}

impl HunterAnims for AnimationController{
    fn play_hunter_throw(&mut self){
        if self.priority > 2 {return}
        self.current_animation = CharacterAnimation::simple(FrameTime::Sequence(vec![0.2, 0.15, 0.2]), vec![4, 5, 6]);
        self.priority = 2;
        self.ticker.to_start();
    }
}


impl Default for AnimationController{
    fn default() -> Self {
        AnimationController{
            animation_speed: 1.,
            ticker: AnimTicker::default(),
            current_animation: CharacterAnimation::simple(FrameTime::Constant(0.5), vec![1]).looped(),
            dir_offset: 7,
            priority: 0,
            direction: 2,
        }
    }
}

impl AnimationController{
    pub fn play_idle(&mut self){
        if self.priority > 0 {return}
        self.play_idle_forced();
    }

    pub fn play_idle_forced(&mut self){
        self.current_animation = CharacterAnimation::simple(FrameTime::Constant(0.5), vec![1]).looped();
        self.priority = 0;
        self.ticker.to_start();
    }

    pub fn play_idle_priority(&mut self, priority: usize){
        if self.priority > priority {return}
        self.current_animation = CharacterAnimation::simple(FrameTime::Constant(0.5), vec![1]).looped();
        self.priority = 0;
        self.ticker.to_start();
    }

    pub fn play_hurt(&mut self){
        self.current_animation = CharacterAnimation::simple(FrameTime::Constant(0.2), vec![3]);
        self.priority = 3;
        self.ticker.to_start();
    }

    pub fn play_walk(&mut self){
        if self.priority >= 1 {return} // prevent walk reloop
        self.current_animation = CharacterAnimation::simple(FrameTime::Constant(0.25), vec![0, 1, 2, 1]).looped();
        self.priority = 1;
        self.ticker.to_start();
    }

    pub fn turn_left(&mut self){
        self.direction = 1;
    }

    pub fn turn_right(&mut self){
        self.direction = 3;
    }

    pub fn turn_up(&mut self){
        self.direction = 2;
    }

    pub fn turn_down(&mut self){
        self.direction = 0;
    }

    pub fn get_idx_and_mirror(&self) -> (usize, bool){
        let Some(idx) = self.ticker.get(&self.current_animation.frame_time, &self.current_animation.frame_idx) else {panic!("Failed to get idx! Probably anim is empty")};
        let mut d = self.direction;
        let mut mirror = false;
        if self.direction == 3 {d = 1; mirror = true} 
        (idx + d * self.dir_offset, mirror)
    }
    
    pub fn tick(&mut self, dt: f32){
        self.ticker.tick(dt * self.animation_speed);
        if self.ticker.frame > self.current_animation.duration{
            if self.current_animation.looped {
                self.ticker.frame %= self.current_animation.duration;
            } else {
                self.play_idle_forced()
            }
        };
    }
}

pub enum FrameTime{
    Constant(f32),
    Sequence(Vec<f32>),
}

pub struct CharacterAnimation {
    frame_time: FrameTime,
    frame_idx: Vec<usize>,
    // umbrella, candle, etc
    item_offsets: Vec<Vec2>,
    eye_offsets: Vec<Vec3>,
    looped: bool,
    duration: f32
}

impl CharacterAnimation {
    pub fn looped(mut self) -> Self{
        self.looped = true;
        self
    }
    pub fn player(frame_time: FrameTime, frame_idx: Vec<usize>, item_offsets: Vec<Vec2>) -> Self {
        if frame_idx.len() == 0 {panic!("Empty animation!")}
        let duration = match &frame_time{
            FrameTime::Constant(t) => {t * frame_idx.len() as f32}
            FrameTime::Sequence(s) => {s.into_iter().sum()}
        };
        CharacterAnimation{
            duration,
            frame_idx,
            frame_time,
            item_offsets,
            eye_offsets: vec![],
            looped: false
        }
    }
    pub fn simple(frame_time: FrameTime, frame_idx: Vec<usize>) -> Self {
        if frame_idx.len() == 0 {panic!("Empty animation!")}
        let duration = match &frame_time{
            FrameTime::Constant(t) => {t * frame_idx.len() as f32}
            FrameTime::Sequence(s) => {s.into_iter().sum()}
        };
        CharacterAnimation{
            duration,
            frame_idx,
            frame_time,
            item_offsets: vec![],
            eye_offsets: vec![],
            looped: false,

        }
    }
    pub fn civilian(frame_time: FrameTime, frame_idx: Vec<usize>, eye_offsets: Vec<Vec3>) -> Self {
        if frame_idx.len() == 0 {panic!("Empty animation!")}
        let duration = match &frame_time{
            FrameTime::Constant(t) => {t * frame_idx.len() as f32}
            FrameTime::Sequence(s) => {s.into_iter().sum()}
        };
        CharacterAnimation{
            duration,
            frame_idx,
            frame_time,
            item_offsets: vec![],
            eye_offsets,
            looped: false,

        }
    }

}

pub(super) fn update_sprites(
    mut commands: Commands,
    mut sprites: Query<(&mut Sprite, &mut TextureAtlas)>,
    mut player_controllers: Query<(Entity, &mut AnimationController, &Children)>,
    //mut civilian_controllers: Query<(Entity, &mut CivilianAnimationController)>,
    //mut hunter_controllers: Query<(Entity, &mut HunterAnimationController)>,
    time: Res<Time>,
){
    let dt = time.delta_seconds();
    for (e, mut c, children) in player_controllers.iter_mut(){
        c.tick(dt);
        for child in children.iter(){
            if let Ok((mut sprite, mut atlas)) = sprites.get_mut(*child){
                let (i, b) = c.get_idx_and_mirror();
                sprite.flip_x = b;
                atlas.index = i;
            }
        }
        
    }
    //for (e, mut c) in civilian_controllers.iter_mut(){c.tick(dt)}
    //for (e, mut c) in hunter_controllers.iter_mut(){c.tick(dt)}
}

