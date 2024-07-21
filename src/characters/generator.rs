use bevy::{math::{uvec2, vec3}, prelude::*};
use rand::Rng;

use super::character::CharacterLayout;

const NPC_BODY : &'static str = "npc/body.png";
const NPC_EYE : &'static str = "npc/eye.png";
const NPC_HAIR : &'static str = "npc/hair.png";
const EYE_RECT_SIZE : UVec2 = uvec2(6, 4);
const HAIR_RECT_SIZE : UVec2 = uvec2(14, 18);
const BODY_RECT_SIZE : UVec2 = uvec2(14, 18);

#[derive(Resource)]
pub struct CharacterGenerator{
    eye_image: Handle<Image>,
    hair_image: Handle<Image>,
    body_image: Handle<Image>,
    eye_layout: Handle<TextureAtlasLayout>,
    hair_layout: Handle<TextureAtlasLayout>,
    body_layout: Handle<TextureAtlasLayout>,
    eye_count : usize,
    hair_count : usize, 
    body_count : usize

}


impl CharacterGenerator {
    fn new(asset_server: &mut AssetServer, layouts: &mut ResMut<Assets<TextureAtlasLayout>>) -> Self {
        let eye_layout = layouts.add(TextureAtlasLayout::from_grid(EYE_RECT_SIZE, 5, 3, Some(uvec2(1, 1)), None));
        let hair_layout = layouts.add(TextureAtlasLayout::from_grid(HAIR_RECT_SIZE, 9, 4, None, None));
        let body_layout = layouts.add(TextureAtlasLayout::from_grid(BODY_RECT_SIZE, 15, 4, None, None));
        let eye_image = asset_server.load(NPC_EYE);
        let hair_image = asset_server.load(NPC_HAIR);
        let body_image = asset_server.load(NPC_BODY);
        let eye_count = 5;
        let hair_count = 9;
        let body_count = 5;
        CharacterGenerator{eye_image, hair_image, body_image, eye_layout, hair_layout, body_layout, eye_count, hair_count, body_count}
    }

    pub fn spawn_random_villain(&self, commands: &mut Commands) -> Entity {
        let layout = CharacterLayout::gen(self.eye_count, self.hair_count, self.body_count);
        commands.spawn((
            SpriteBundle{
                texture: self.body_image.clone(),
                ..default()
            },
            TextureAtlas{
                layout: self.body_layout.clone(),
                index: layout.get_animation_index()
            },
            layout,
            Name::new("Villain")
        )).with_children(|commands|{
            commands.spawn((
                SpriteBundle{
                    texture: self.hair_image.clone(),
                    ..default()
                },
                TextureAtlas{
                    layout: self.hair_layout.clone(),
                    index: rand::thread_rng().gen_range(0..self.hair_count)
                },
                Name::new("Hair")
            )).insert(Transform::from_translation(vec3(0., 0., 0.2)));
            commands.spawn((
                SpriteBundle{
                    texture: self.eye_image.clone(),
                    ..default()
                },
                TextureAtlas{
                    layout: self.eye_layout.clone(),
                    index: rand::thread_rng().gen_range(0..self.eye_count)
                },
                Name::new("Eye")
            )).insert(Transform::from_translation(vec3(0., -1., 0.1)));
        }).id()
    }
}




fn setup(
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut asset_server: ResMut<AssetServer>,
){
    commands.insert_resource(CharacterGenerator::new(&mut asset_server, &mut layouts));
}


pub struct CharacterGeneratorPlugin;

impl Plugin for CharacterGeneratorPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}
