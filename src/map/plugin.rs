use std::time::Duration;

use bevy::{math::ivec2, prelude::*, utils::HashSet};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::{KinematicCharacterController, Velocity};

use crate::player::components::Player;

use super::tilemap::{self, TileObsticle, TransformToGrid};

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
        app.add_systems(Update, (tilemap::spawn_tile_collision, update_unit_grid, tilemap::spawn_tile_tree));
        app.add_systems(PostUpdate, trespassable_spawn_listener);
        app.add_systems(PreUpdate, sizif);
        app.register_ldtk_entity::<EntitySpawnerBundle>("EnemySpawner");

        app.register_ldtk_int_cell_for_layer::<tilemap::TileObsticleBundle>("Ground", 1);
        app.register_ldtk_int_cell_for_layer::<tilemap::TiledTreeBundle>("Ground", 3);
        app.register_ldtk_int_cell_for_layer::<tilemap::TileObsticleBundle>("Ground", 4);

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
    pub cells: Vec<Vec<bool>>,
    pub units: HashSet<IVec2>,
    pub ready: bool
}

impl TrespassableCells {
    pub fn is_trespassable(&self, pos: &IVec2) -> bool{
        let Some(column) = self.cells.get(pos.x as usize) else {return false};
        let Some(value) = column.get(pos.y as usize) else {return false};
        *value
    }
}


fn update_unit_grid(
    mut trespassable: ResMut<TrespassableCells>,
    transfromer: Res<TransformToGrid>,
    units_q: Query<&Transform, (With<Velocity>, Without<Player>)>
){
    trespassable.units.clear();
    for t in units_q.iter(){
        let pos = transfromer.from_world_i32(t.translation.xy());
        trespassable.units.insert(pos);
    }
}

fn trespassable_spawn_listener(
    //mut commands: Commands,
    entity_q: Query<&GridCoords, Added<TileObsticle>>,
    mut trespassable_cells: ResMut<TrespassableCells>,
    transfromer: Res<TransformToGrid>,
    //level_query: Query<(Entity, &LevelIid)>,
    //ldtk_projects: Query<&Handle<LdtkProject>>,
    //ldtk_project_assets: Res<Assets<LdtkProject>>,
){
    if !entity_q.is_empty() && transfromer.ready {
        let cells_column = vec![true; transfromer.grid_size.y as usize];
        let mut cells_grid = vec![cells_column; transfromer.grid_size.x as usize];
        
        for coords in entity_q.iter(){
            let pos = ivec2(coords.x, transfromer.grid_size.y - coords.y - 1);
            cells_grid[pos.x as usize][pos.y as usize] = false;
        }

        trespassable_cells.cells = cells_grid;
        info!("Trespassable cells inited!");
        trespassable_cells.ready = true;
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