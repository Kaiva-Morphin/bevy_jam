use bevy::{math::vec2, prelude::*, utils::{HashMap, HashSet}};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{dynamics::RigidBody, geometry::{Collider, Friction}};

pub fn pre_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    let ldtk_handle = asset_server.load("testmap.ldtk");
    commands.spawn(LdtkWorldBundle {
        ldtk_handle,
        transform: Transform::from_translation(Vec3::Z * -10.),
        ..Default::default()
    });
    commands.insert_resource(TransformToGrid{
        height: 0.,
        transform: vec2(0., 0.),
        grid_size: vec2(16., 16.),
        ready: false
    });
}

pub fn watcher (
    mut commands: Commands,
    res: Res<TransformToGrid>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
){
    if res.ready {return;}
    for (level_iid, level_transform) in levels.iter() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("ldtk project should be loaded before player is spawned");
        let level = ldtk_project
            .get_raw_level_by_iid(level_iid.get())
            .expect("level should exist in only project");
        commands.insert_resource(TransformToGrid{
            height: level.px_hei as f32,
            transform: level_transform.translation().xy(),
            grid_size: vec2(16., 16.),
            ready: true
        });
        return;
    }
}

#[derive(Resource)]
pub struct TransformToGrid{
    height: f32,
    transform: Vec2,
    grid_size: Vec2,
    ready: bool
}


impl TransformToGrid{
    pub fn from_world(&self, position: Vec2) -> Vec2{
        ((vec2(0., self.height) + self.transform) - position) / self.grid_size * vec2(-1., 1.)
    }    
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct TileObsticle;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TileObsticleBundle {
    obsticle: TileObsticle,
}

#[derive(Resource)]
pub struct GridSize {
    pub size: IVec2,
}

/// Spawns heron collisions for the walls of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_tile_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<TileObsticle>>,
    parent_query: Query<&Parent, Without<TileObsticle>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut grid_size_res: ResMut<GridSize>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];
                grid_size_res.size = IVec2::new(level.layer_instances()[0].c_wid, level.layer_instances()[0].c_hei);
                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(Name::new("COLLIDER"))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}























