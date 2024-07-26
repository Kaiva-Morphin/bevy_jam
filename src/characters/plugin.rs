use bevy::prelude::*;

use super::{animation::update_sprites, generator::{CharacterGenerator, CharacterGeneratorPlugin}};



pub struct CharacterPluginsBundle;

impl Plugin for CharacterPluginsBundle{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            CharacterAnimationPlugin, 
            CharacterGeneratorPlugin
        ));
    }
}



pub struct CharacterAnimationPlugin;

impl Plugin for CharacterAnimationPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_sprites.run_if(in_state(crate::systems::GameState::InGame)));
    }
}



pub struct CharacterGeneratorViewerPlugin;

impl Plugin for CharacterGeneratorViewerPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CharacterGeneratorPlugin);
        app.add_systems(Update, view_generation);
    }
}


#[derive(Component)]
struct GenerationResult;


fn view_generation(
    gen_q: Query<Entity, With<GenerationResult>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    generator: Res<CharacterGenerator>
){
    if keyboard.pressed(KeyCode::Space){
        for e in gen_q.iter(){
            commands.entity(e).despawn_recursive();
        }
        let e = generator.spawn_random_villain(&mut commands);
        commands.entity(e).insert(GenerationResult);
    }

}



