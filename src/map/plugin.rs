use std::time::Duration;

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
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_systems(PreStartup, tilemap::pre_setup)
        .add_systems(Update, tilemap::watcher)
        .add_systems(Update, tilemap::spawn_tile_collision)
        .add_systems(PostUpdate, trespassable_spawn_listener)
        .add_systems(PreUpdate, sizif)
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


#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct EntitySpawner {
    pub timer: Timer,
}

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

fn sizif(
    mut commands: Commands,
    q: Query<Entity, Added<EntitySpawner>>,
    ){
    for e in q.iter(){
    commands.entity(e).insert(EntitySpawner{ timer: Timer::new(Duration::from_secs_f32(1.), TimerMode::Repeating)});
    }
    }