use bevy::prelude::*;
use bevy::app::Plugin;






pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        
    }
}



fn setup(
    mut commands: Commands
){
    commands.spawn(
        NodeBundle{
            style: Style{
                ..default()
            },
            ..default()
        }
    );
}



fn update(

){

}





