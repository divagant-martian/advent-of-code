use crate::{Tile, TileId};
use std::collections::BTreeMap;
use std::fs::read_to_string;

pub fn parse_tile(data: &str) -> (TileId, Tile) {
    let mut lines = data.lines();
    let tile_id: TileId = lines
        .next()
        .expect("tile has a tile id line")
        .split(|c| c == ' ' || c == ':')
        .nth(1)
        .expect("tile id line has id")
        .parse()
        .unwrap();
    let tiles = lines
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect();

    (tile_id, tiles)
}

pub fn parse_tile_file(path: &str) -> BTreeMap<TileId, Tile> {
    read_to_string(path)
        .expect("bad file")
        .split("\n\n")
        .map(|part| parse_tile(part))
        .collect()
}
