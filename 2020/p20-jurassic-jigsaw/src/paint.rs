use crate::Tile;
use std::collections::HashSet;

#[allow(clippy::ptr_arg)]
pub fn paint_tile(tile: &Tile) {
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
            } else {
                paint.push(' ');
                paint.push(' ');
            }
        }
        paint.push('\n');
    }

    println!("{}", paint);
}

pub fn paint_tile_with_monsters(
    tile: &HashSet<(usize, usize)>,
    tiles_with_monsters: &HashSet<(usize, usize)>,
) {
    let (max_x, max_y) = tile
        .iter()
        .fold((0_usize, 0_usize), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });

    let mut paint = String::with_capacity(max_y * max_x);
    for y in 0..=max_y {
        for x in 0..=max_x {
            if tiles_with_monsters.contains(&(x, y)) {
                paint.push('█');
                paint.push('█');
            } else if tile.contains(&(x, y)) {
                paint.push('~');
                paint.push('~');
            } else {
                paint.push(' ');
                paint.push(' ');
            }
        }
        paint.push('\n');
    }

    println!("{}", paint);
}
