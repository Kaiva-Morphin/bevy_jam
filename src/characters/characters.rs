use bevy::prelude::*;
use rand::{self, Rng};


#[derive (Component) ]
pub struct CharacterLayout {
    eye_count: usize,
    body_count: usize,
    hair_count: usize,
    eye_variant: usize,
    body_variant: usize,
    hair_variant: usize,
}

impl CharacterLayout {
    pub fn gen(eye_count: usize, hair_count: usize, body_count: usize) -> Self {
        CharacterLayout{
            eye_count,
            eye_variant: rand::thread_rng().gen_range(0..eye_count), 
            hair_count,
            hair_variant: rand::thread_rng().gen_range(0..hair_count), 
            body_count,
            body_variant: rand::thread_rng().gen_range(0..body_count), 
        }
    }
    pub fn get_animation_index(&self) -> usize{
        self.body_variant * 3 + 1
    }
}







