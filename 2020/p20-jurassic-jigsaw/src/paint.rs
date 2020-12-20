use crate::op::{Op, Operation};
use crate::{Tile, TileId};
use std::collections::BTreeMap;

pub fn paint_tile(tile: &Tile) -> String {
    let (max_x, max_y) = tile
        .iter()
        .fold((0_usize, 0_usize), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });

    let mut paint = String::with_capacity(max_y * max_x);
    for y in 0..=max_y {
        for x in 0..=max_x {
            if tile.contains(&(x, y)) {
                paint.push('█');
                paint.push('█');
                paint.push('█');
            } else {
                paint.push(' ');
                paint.push(' ');
                paint.push(' ');
            }
        }
        paint.push('\n');
    }

    paint
}

pub fn paint_puzzle(
    puzzle: &BTreeMap<(usize, usize), TileId>,
    tiles: &BTreeMap<TileId, Tile>,
    op: &Op,
) {
    let min_x = puzzle.keys().map(|pos| pos.0).min().unwrap();
    let min_y = puzzle.keys().map(|pos| pos.1).min().unwrap();

    let size = 11;
    let mut flatened_puzzle = Vec::new();
    for ((mut puzzle_x, mut puzzle_y), tile_id) in puzzle {
        let tile = tiles.get(tile_id).unwrap();
        puzzle_x -= min_x;
        puzzle_y -= min_y;
        for (local_x, local_y) in tile {
            flatened_puzzle.push((size * puzzle_x + local_x, size * puzzle_y + local_y));
        }
    }

    op.operate(&mut flatened_puzzle);
    let paint = paint_tile(&flatened_puzzle);
    println!("Puzzle\n{}", paint);
}
