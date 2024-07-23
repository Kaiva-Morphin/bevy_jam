use bevy::{math::{ivec2, uvec2, IVec2}, utils::hashbrown::HashSet};
use pathfinding::prelude::astar;







#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
  fn distance(&self, other: &Pos) -> u32 {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
  }

  fn successors(&self) -> Vec<(Pos, u32)> {
    let &Pos(x, y) = self;
    vec![Pos(x+1,y), Pos(x-1,y), Pos(x,y+1), Pos(x,y-1)]
         .into_iter().map(|p| (p, 1)).collect()
  }
  fn ivec2(vec: &IVec2) -> Self{
    Pos(vec.x, vec.y)
  }
}




fn main() {
    let bitmap = vec![
        vec![1, 1, 1, 1, 1, 1],
        vec![0, 1, 0, 0, 1, 1],
        vec![1, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 1],
    ];
    let mut allowed_cells = HashSet::new();
    for (y, y_arr) in bitmap.iter().enumerate(){
        for (x, walkable) in y_arr.iter().enumerate(){
            if *walkable == 1 {
                allowed_cells.insert(Pos(x as i32, y as i32));
            }
        }
    }

    

    let start = Pos(0, 0);
    let GOAL = Pos(0, 3);
    let result = astar(
        &start,
        |p| {
            let mut moves = vec![];
            let pos = ivec2(p.0, p.1);
            //let has = |val: &IVec2| allowed_cells.contains(val);
            let mov = Pos(p.0 + 1, p.1);
            if allowed_cells.contains(&mov){moves.push((mov, 1));};
            let mov = Pos(p.0 - 1, p.1);
            if allowed_cells.contains(&mov){moves.push((mov, 1));};
            let mov = Pos(p.0, p.1 + 1);
            if allowed_cells.contains(&mov){moves.push((mov, 1));};
            let mov = Pos(p.0, p.1 - 1);
            if allowed_cells.contains(&mov){moves.push((mov, 1));};


            moves
        },
        |p| p.distance(&GOAL), //  / 3
        |p| *p == GOAL

    );
    let Some(path) = result else {panic!("no solution!")};
    for part in path.0.iter(){
        println!("{:?}", part)
    }
}