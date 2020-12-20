use crate::border::BORDERS;
use crate::op::{Op, Operation, OPERATIONS};
use crate::parse::parse_tile_file;
use std::collections::HashMap;
use std::collections::VecDeque;

mod border;
mod op;
mod paint;
mod parse;

pub type TileId = usize;
pub type Position = (usize, usize);
pub type Tile = Vec<Position>;

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let tiles = parse_tile_file(&path);
    let real_pic = get_flattend_pic(tiles);
}

fn get_flattend_pic(mut unasigned_tiles: HashMap<TileId, Tile>) -> Tile {
    let mut assigned_tiles: HashMap<TileId, Tile> = HashMap::new();
    let mut search_queue = VecDeque::new();
    let mut puzzle: HashMap<(usize, usize), TileId> = HashMap::new();

    let first_position = (100, 100);
    search_queue.push_back(first_position);

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
    let big_flattened = flatten_puzzle(&puzzle, &assigned_tiles);
    paint::paint_tile(&big_flattened);

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

    // Part 2: remove the borders of each tile
    let size = assigned_tiles
        .values()
        .map(|list| list.iter().map(|pos| pos.0).max().unwrap())
        .max()
        .unwrap();
    for tile in assigned_tiles.values_mut() {
        tile.retain(|&(x, y)| x != 0 && x != size && y != 0 && y != size);
    }

    flatten_puzzle(&puzzle, &assigned_tiles)
}

fn flatten_puzzle(puzzle: &HashMap<(usize, usize), TileId>, tiles: &HashMap<TileId, Tile>) -> Tile {
    let min_x = puzzle.keys().map(|pos| pos.0).min().unwrap();
    let min_y = puzzle.keys().map(|pos| pos.1).min().unwrap();

    let size = tiles
        .values()
        .map(|list| list.iter().map(|pos| pos.0).max().unwrap())
        .max()
        .unwrap();
    let mut flatened_puzzle = Vec::new();
    for ((mut puzzle_x, mut puzzle_y), tile_id) in puzzle {
        let tile = tiles.get(tile_id).unwrap();
        puzzle_x -= min_x;
        puzzle_y -= min_y;
        for (local_x, local_y) in tile {
            flatened_puzzle.push((size * puzzle_x + local_x, size * puzzle_y + local_y));
        }
    }
    flatened_puzzle
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
