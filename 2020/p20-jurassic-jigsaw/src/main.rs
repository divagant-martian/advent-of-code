#[allow(unused)]
use crate::op::{Flip, Op, Operation, Rotate, OPERATIONS};
#[allow(unused)]
use crate::paint::paint_tile;
use crate::parse::parse_tile_file;
use std::collections::HashMap;
use std::collections::VecDeque;

mod op;
mod paint;
mod parse;

pub type TileId = usize;
pub type Position = (usize, usize);
pub type Tile = Vec<Position>;

#[derive(Debug)]
enum Border {
    U,
    D,
    R,
    L,
}
const BORDERS: [Border; 4] = [Border::R, Border::L, Border::U, Border::D];

impl Border {
    fn get(&self, tile: &Tile) -> Vec<usize> {
        let max_x = tile.iter().map(|p| p.0).max().unwrap();
        let max_y = tile.iter().map(|p| p.1).max().unwrap();
        let mut border: Vec<usize> = tile
            .iter()
            .filter(|(x, y)| match self {
                Border::U => *y == 0,
                Border::D => *y == max_y,
                Border::L => *x == 0,
                Border::R => *x == max_x,
            })
            .map(|(x, y)| match self {
                Border::U | Border::D => x,
                Border::L | Border::R => y,
            })
            .cloned()
            .collect();
        border.sort_unstable();
        border
    }

    fn oposite(&self) -> Self {
        match self {
            Border::U => Border::D,
            Border::D => Border::U,
            Border::R => Border::L,
            Border::L => Border::R,
        }
    }

    fn position_at_border(&self, placement: &(usize, usize)) -> (usize, usize) {
        match self {
            Border::U => (placement.0, placement.1 - 1),
            Border::D => (placement.0, placement.1 + 1),
            Border::R => (placement.0 + 1, placement.1),
            Border::L => (placement.0 - 1, placement.1),
        }
    }
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut unasigned_tiles = parse_tile_file(&path);
    let mut assigned_tiles: HashMap<TileId, Tile> = HashMap::new();
    let mut search_queue = VecDeque::new();
    let mut puzzle: HashMap<(usize, usize), TileId> = HashMap::new();

    let first_position = (100, 100);
    search_queue.push_back(first_position);

    let final_op = Op {
        rotate: None,
        flip: None,
    };

    // assuming the first one that fits is one we can use (this worked fortunately)
    while let Some(position) = search_queue.pop_front() {
        if puzzle.contains_key(&position) {
            continue;
        }
        let mut found = None;
        for (maybe_id, maybe_tile) in &unasigned_tiles {
            if let Some(modified) =
                fits_at_position(maybe_tile, &puzzle, &assigned_tiles, &position)
            {
                found = Some((*maybe_id, modified));
                break;
            }
        }

        if let Some((id, tile)) = found {
            puzzle.insert(position, id);
            assigned_tiles.insert(id, tile);
            unasigned_tiles.remove(&id);

            for border in &BORDERS {
                let to_check = border.position_at_border(&position);
                if !puzzle.contains_key(&to_check) {
                    search_queue.push_back(to_check);
                }
            }
        }
    }
    assert!(unasigned_tiles.is_empty(), "All tiles must be used");
    paint::paint_puzzle(&puzzle, &assigned_tiles, &final_op);

    // We completed the puzzle, not let's find the corners
    let part_1: usize = puzzle
        .iter()
        .filter_map(|(position, tile_id)| {
            if BORDERS
                .iter()
                .filter(|border| puzzle.contains_key(&border.position_at_border(position)))
                .count()
                == 2
            {
                println!("corner {}", tile_id);
                Some(tile_id)
            } else {
                None
            }
        })
        .product();
    println!("Part 1 {}", part_1);
}

fn fits_at_position(
    maybe_tile: &Tile,
    puzzle: &HashMap<(usize, usize), TileId>,
    assigned_tiles: &HashMap<TileId, Tile>,
    placement: &(usize, usize),
) -> Option<Tile> {
    // check if there is one operation that makes it fit everywhere
    for op in &OPERATIONS {
        let modified = op.operate_clone(maybe_tile);
        let mut fits = true;
        for border in &BORDERS {
            let to_check = border.position_at_border(placement);
            if let Some(fixed_id) = puzzle.get(&to_check) {
                let fixed_tile = assigned_tiles.get(fixed_id).unwrap();
                if border.get(&modified) != border.oposite().get(fixed_tile) {
                    fits = false;
                    break;
                }
            }
        }
        if fits {
            return Some(modified);
        }
    }
    None
}
