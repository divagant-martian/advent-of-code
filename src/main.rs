use std::collections::HashMap;
use std::fs::read_to_string;
use Tile::*;

#[derive(Copy, Clone, Debug)]
enum Tile {
    Key(char),
    Door(char),
    Empty,
    Me,
}

fn get_data(path: &str) -> HashMap<(usize, usize), Tile> {
    let raw = read_to_string(path).expect("problem with file");
    raw.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices().flat_map(move |(col, c)| {
                let pos = (row, col);
                match c {
                    'a'..='z' => Some((pos, Key(c))),
                    'A'..='Z' => Some((pos, Door(c))),
                    '.' => Some((pos, Empty)),
                    '@' => Some((pos, Me)),
                    _ => None, // ignore walls
                }
            })
        })
        .collect()
}

fn main_dijkstra(data: HashMap<(usize, usize), Tile>) {
    // find source
}

fn main() {
    let data = get_data("data/test0.txt");
    println!(
        "{}",
        data.iter()
            .map(|t| format!("{:<2?} {:?}\n", t.0, t.1))
            .collect::<String>()
    );
}
