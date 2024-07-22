use bevy::{color::palettes::css::RED, math::ivec2, prelude::*, utils::HashSet};
use pathfinding::prelude::{astar, bfs};
use std::vec::IntoIter;
use crate::{map::plugin::TrespassableCells, player::components::Player};

use super::components::Hunter;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Pos(IVec2);

const MOVES: [IVec2; 4] = [
    ivec2(1, 0),
    ivec2(1, 1),
    ivec2(-1, 0),
    ivec2(-1, -1),
];

impl Pos {
    fn successors(&self, trespassable: &HashSet<IVec2>) -> impl Iterator<Item = (Pos, i32)> {
        let &Pos(pos) = self;
        let mut moves = Vec::with_capacity(4);
        for mov in MOVES {
            if trespassable.contains(&(pos + mov)) {
                moves.push(Pos(mov))
            }
        }
        moves.into_iter().map(|p| (p, 1)) // second index? recalc weight
    }
    fn weight(&self, end: &Pos) -> i32{
        (self.0.x - end.0.x).abs() + (self.0.y - end.0.y).abs()
    }
}

pub fn process_pathfinding(
    player_transform: Query<&Transform, (With<Player>, Without<Hunter>)>,
    mut hunters_transform: Query<&mut Transform, With<Hunter>>,
    trespassable: Res<TrespassableCells>,
    mut gizmos: Gizmos,
) {
    if trespassable.ready {
        println!("ready");
        let player_ipos = (player_transform.single().translation.xy() / 16.).as_ivec2();
        for hunter_transform in hunters_transform.iter_mut() {
            let hunter_ipos = (hunter_transform.translation.xy() / 16.).as_ivec2();
            if let Some(path) = find_path(&Pos(hunter_ipos), &Pos(player_ipos), &trespassable) {
                println!("{:?}", path);
                for id in 0..path.len() - 1 {
                    let p0 = path[id].0.as_vec2();
                    let p1 = path[id + 1].0.as_vec2();
    
                    gizmos.line_2d(p0, p1, Color::Srgba(RED))
                }
            }
        }
    }
}

pub fn find_path(
    start: &Pos,
    end: &Pos,
    trespassable: &Res<TrespassableCells>,
) -> Option<Vec<Pos>>{
    if let Some(path) = astar(
    start,
    |start| start.successors(&trespassable.cells),
    |start| start.weight(end),
    |start| start == end)
    {
        println!("did sth");
        return Some(path.0)
    }
    None
}