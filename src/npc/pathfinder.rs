use bevy::{math::ivec2, prelude::*, utils::HashSet};
use pathfinding::prelude::astar;
use crate::map::{plugin::TrespassableCells, tilemap::TransformToGrid};

use super::components::NpcState;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos(IVec2);

const MOVES: [IVec2; 4] = [
    ivec2(1, 0),
    ivec2(0, 1),
    ivec2(-1, 0),
    ivec2(0, -1),
];

impl Pos {
    fn successors(&self, trespassable: &Vec<Vec<bool>>) -> impl Iterator<Item = (Pos, i32)> {
        let is_trasspassable = |pos: &IVec2| -> bool {
            let Some(column) = trespassable.get(pos.x as usize) else {return false};
            let Some(value) = column.get(pos.y as usize) else {return false};
            *value
        };

        let &Pos(pos) = self;
        let mut moves = Vec::with_capacity(4);
        for mov in MOVES {
            let t = pos + mov;
            if is_trasspassable(&t) {
                moves.push(t)
            }
        }
        let mut hardmoves = HashSet::new();
        for i in 0..moves.len() {
            for j in i + 1..moves.len() {
                let hardmove = moves[i] + moves[j] - pos;
                if is_trasspassable(&hardmove) {
                    hardmoves.insert(hardmove);
                }
            }
        }
        let mut out = Vec::with_capacity(8);
        for i in moves {
            out.push((Pos(i), 10))
        }
        for i in hardmoves {
            out.push((Pos(i), 14))
        }
        out.into_iter()
    }
    fn weight(&self, end: &Pos) -> i32{
        (self.0.x - end.0.x).abs() + (self.0.y - end.0.y).abs() * 10
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
            NpcState::Chill => {
                if let Some(path) = find_path_huncha(&Pos(start_ipos), &Pos(end_ipos), trespassable) {
                    if path.len() > 1 {
                        return Some(path.into_iter().map(|x| x.0).collect());
                    } else {
                        return None;
                    }
                }
            }
            NpcState::Look => {
                if let Some(path) = find_path_huncha(&Pos(start_ipos), &Pos(end_ipos), trespassable) {
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
    |p| 9999 - p.0.distance_squared(end.0),
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