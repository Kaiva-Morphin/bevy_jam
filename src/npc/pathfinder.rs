use bevy::{color::palettes::css::RED, math::ivec2, prelude::*, utils::{hashbrown::HashSet}};
use bevy_ecs_ldtk::utils::translation_to_grid_coords;
use pathfinding::prelude::{astar, bfs};
use std::vec::IntoIter;
use crate::{map::{plugin::TrespassableCells, tilemap::{TransformToGrid}}, player::components::Player};

use super::components::{Hunter, NpcPath, NpcState};

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
                let hardmove = moves[i] + moves[j] - pos;
                if trespassable.contains(&hardmove) {
                    hardmoves.insert(hardmove);
                }
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

pub fn pathfinder(
    start_ipos: IVec2,
    end_ipos: IVec2,
    trespassable: &Res<TrespassableCells>,
    transformer: &Res<TransformToGrid>,
    npc_state: NpcState,
) -> Option<Vec<IVec2>> {
    if trespassable.ready && transformer.ready {
        match npc_state {
            NpcState::Chase => {
                if let Some(path) = find_path_huncha(&Pos(start_ipos), &Pos(end_ipos), trespassable) {
                    if path.len() > 5 {
                        return Some(path[0..path.len() - 4].into_iter().map(|x| x.0).collect());
                    } else {
                        return None;
                    }
                }
            }
            NpcState::Escape => {
                if let Some(path) = find_path_hunesc(&Pos(start_ipos), &Pos(end_ipos), trespassable) {
                    if path.len() > 1 {
                        return Some(path.into_iter().map(|x| x.0).collect());
                    } else {
                        return None;
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn find_path_hunesc(
    start: &Pos,
    end: &Pos,
    trespassable: &Res<TrespassableCells>,
) -> Option<Vec<Pos>>{
    if let Some(path) = astar(
    start,
    |p| p.successors(&trespassable.cells),
    |p| 999 - p.weight(end),
    |p| p.0.distance_squared(end.0) > 25)
    {
        return Some(path.0)
    }
    None
}

fn find_path_huncha(
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
        return Some(path.0)
    }
    None
}