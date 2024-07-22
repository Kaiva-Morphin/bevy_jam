use bevy::{math::ivec2, prelude::*, utils::HashSet};
use bevy_ecs_ldtk::prelude::*;

use super::tilemap;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LdtkPlugin,
        ));
        app.insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(Startup, tilemap::setup)
        .add_systems(Update, (tilemap::spawn_tile_collision, spawner_spawn_listener, trespassable_spawn_listener))
        .register_ldtk_entity::<EntitySpawnerBundle>("EnemySpawner")
        .register_ldtk_int_cell::<tilemap::TileObsticleBundle>(1)
        .register_ldtk_int_cell::<TrespassableCellBundle>(2);
        //.register_ldtk_int_cell::<components::LadderBundle>(2)
        //.register_ldtk_int_cell::<components::WallBundle>(3)
        //.register_ldtk_entity::<components::PlayerBundle>("Player")
        //.register_ldtk_entity::<components::MobBundle>("Mob")
        //.register_ldtk_entity::<components::PumpkinsBundle>("Pumpkins")
        app.insert_resource(TrespassableCells::default());
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct TrespassableCell;


#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TrespassableCellBundle{
    cell: TrespassableCell
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct EntitySpawner;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct EntitySpawnerBundle {
    spanwer: EntitySpawner,
}

#[derive(Resource, Default)]
pub struct TrespassableCells{
    pub cells: HashSet<IVec2>,
    pub ready: bool
}

fn trespassable_spawn_listener(
    //mut commands: Commands,
    entity_q: Query<&GridCoords, Added<TrespassableCell>>,
    mut cells: ResMut<TrespassableCells>,
    //level_query: Query<(Entity, &LevelIid)>,
    //ldtk_projects: Query<&Handle<LdtkProject>>,
    //ldtk_project_assets: Res<Assets<LdtkProject>>,
){

    if !entity_q.is_empty(){
        for coords in entity_q.iter(){
            cells.cells.insert(ivec2(coords.x, coords.y));
        }
        println!("ADDED! {} cells", cells.cells.len());
        cells.ready = true;
    }
    
    
}



fn spawner_spawn_listener( // todo: rm
    entity_q: Query<&GridCoords, With<EntitySpawner>>,
){
    for coords in entity_q.iter(){
        println!("Spawned enemy spawner {:?}", coords);
    }
}

