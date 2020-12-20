#[allow(unused)]
use crate::op::{Flip, Op, Operation, Rotate, OPERATIONS};
#[allow(unused)]
use crate::paint::paint_tile;
use crate::parse::parse_tile_file;
use std::collections::BTreeMap;
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
        let (max_x, max_y) = tile.iter().max().cloned().unwrap();
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
}

fn matches_border(
    fixed_tile: &Tile,
    searched_border: &Border,
    movable_tile: &Tile,
) -> Option<Tile> {
    let fixed_border = searched_border.get(fixed_tile);
    let matching_border = searched_border.oposite();
    for op in &OPERATIONS {
        let moved_tile = op.operate_clone(movable_tile);
        let border_to_check = matching_border.get(&moved_tile);
        if fixed_border == border_to_check {
            return Some(moved_tile);
        }
    }

    None
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let mut unasigned_tiles = parse_tile_file(&path);
    let mut assigned_tiles: BTreeMap<TileId, Tile> = BTreeMap::new();
    let mut search_queue = VecDeque::new();
    let mut puzzle: BTreeMap<(usize, usize), TileId> = BTreeMap::new();

    // put the first tile in the middle of the board
    let first_id = unasigned_tiles.keys().next().cloned().unwrap();
    let first_tile = unasigned_tiles.remove(&first_id).unwrap();
    let first_position = (100, 100);
    puzzle.insert(first_position, first_id);
    assigned_tiles.insert(first_id, first_tile);
    search_queue.push_back((first_id, first_position.0, first_position.1));

    let final_op = Op {
        rotate: None,
        flip: None,
    };

    while let Some((current_fixed_id, placement_x, placement_y)) = search_queue.pop_front() {
        // get the tile
        let current_tile = assigned_tiles.get(&current_fixed_id).cloned().unwrap();

        // search for each side a matching tile
        for border in &BORDERS {
            // check first is this slot is occupied
            let puzzle_position = match border {
                Border::D => (placement_x, placement_y + 1),
                Border::U => (placement_x, placement_y - 1),
                Border::R => (placement_x + 1, placement_y),
                Border::L => (placement_x - 1, placement_y),
            };

            if puzzle.contains_key(&puzzle_position) {
                continue;
            }

            // we need to check this border

            let mut found = None;
            for (maybe_id, maybe_tile) in &unasigned_tiles {
                if let Some(moved_tile) = matches_border(&current_tile, border, maybe_tile) {
                    found = Some((*maybe_id, moved_tile));
                    break;
                }
            }

            if let Some((found_id, modified_tile)) = found {
                // we found one
                unasigned_tiles.remove(&found_id);
                assigned_tiles.insert(found_id, modified_tile);
                search_queue.push_back((found_id, puzzle_position.0, puzzle_position.1));
                assert!(puzzle.insert(puzzle_position, found_id).is_none());
                if assigned_tiles.len() > 100 {
                    paint::paint_puzzle(&puzzle, &assigned_tiles, &final_op);
                    println!("{}", "\n".repeat(5));
                    std::thread::sleep_ms(600);
                }
            }
        }
    }
}
