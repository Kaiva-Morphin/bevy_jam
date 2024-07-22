use bevy::{color::palettes::css::RED, math::ivec2, prelude::*, utils::{hashbrown::HashSet}};
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use pathfinding::prelude::{astar, bfs};
use std::vec::IntoIter;
use crate::{map::{plugin::TrespassableCells, tilemap::{TransformToGrid}}, player::components::Player};

use super::components::{Hunter, NpcPath};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos(IVec2);

const MOVES: [IVec2; 4] = [
    ivec2(1, 0),
    ivec2(0, 1),
    ivec2(-1, 0),
    ivec2(0, -1),
];

const DIAGMOVES: [IVec2; 4] = [
    ivec2(1, 1),
    ivec2(1, -1),
    ivec2(-1, 1),
    ivec2(-1, -1),
];

impl Pos {
    fn successors(&self, trespassable: &HashSet<IVec2>) -> impl Iterator<Item = (Pos, i32)> {
        let &Pos(pos) = self;
        let mut moves = Vec::with_capacity(4);
        for mov in MOVES {
            let t = pos + mov;
            if trespassable.contains(&t) {
                moves.push(t)
            }
        }
        let mut hardmoves = HashSet::new();
        for i in 0..moves.len() {
            for j in i + 1..moves.len() {
                hardmoves.insert(moves[i] + moves[j] - pos);
            }
        }
        let mut out = Vec::with_capacity(8);
        for i in moves {
            out.push((Pos(i), 2))
        }
        for i in hardmoves {
            out.push((Pos(i), 3))
        }
        out.into_iter()
    }
    fn weight(&self, end: &Pos) -> i32{
        (self.0.x - end.0.x).abs() + (self.0.y - end.0.y).abs() * 5
    }
}

pub fn process_pathfinding(
    player_transform: Query<&Transform, (With<Player>, Without<Hunter>)>,
    mut hunters_transform: Query<(&mut Transform, &mut NpcPath), With<Hunter>>,
    trespassable: Res<TrespassableCells>,
    transformer: Res<TransformToGrid>,
) {
    if trespassable.ready && transformer.ready {
        let player_pos = player_transform.single().translation.xy();
        let player_ipos = transformer.from_world(player_pos).as_ivec2();
        for (hunter_transform, mut hunter_path) in hunters_transform.iter_mut() {
            let hunter_pos = hunter_transform.translation.xy();
            let hunter_ipos = transformer.from_world(hunter_pos).as_ivec2();
            if let Some(path) = find_path(&Pos(hunter_ipos), &Pos(player_ipos), &trespassable) {
                hunter_path.path = path.into_iter().map(|x| x.0.as_vec2()).collect();
            }
        }
    }
}

fn find_path(
    start: &Pos,
    end: &Pos,
    trespassable: &Res<TrespassableCells>,
) -> Option<Vec<Pos>>{
    if let Some(path) = astar(
    start,
    |p| p.successors(&trespassable.cells),
    |p| p.weight(end),
    |p| p == end)
    {
        // println!("did sth");
        return Some(path.0)
    }
    None
}