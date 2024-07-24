use bevy::{math::ivec2, prelude::*, utils::HashSet};
use bevy_ecs_ldtk::prelude::*;

use super::tilemap::{self, TransformToGrid};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            LdtkPlugin,
        ));
        app.insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        });
        app.add_systems(PreStartup, tilemap::pre_setup);
        app.add_systems(Update, tilemap::watcher);
        app.add_systems(Update, (tilemap::spawn_tile_collision, spawner_spawn_listener));
        app.add_systems(PostUpdate, trespassable_spawn_listener);

        //app.register_ldtk_entity::<EntitySpawnerBundle>("EnemySpawner");

        app.register_ldtk_int_cell::<tilemap::TileObsticleBundle>(1);
        app.register_ldtk_int_cell::<tilemap::TileObsticleBundle>(3);
        app.register_ldtk_int_cell::<tilemap::TileObsticleBundle>(4);

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
    transfromer: Res<TransformToGrid>,
    //level_query: Query<(Entity, &LevelIid)>,
    //ldtk_projects: Query<&Handle<LdtkProject>>,
    //ldtk_project_assets: Res<Assets<LdtkProject>>,
){
    if !entity_q.is_empty() && transfromer.ready {
        for coords in entity_q.iter(){
            cells.cells.insert(ivec2(coords.x, transfromer.grid_size.y - coords.y - 1));
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

