mod state;
use state::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::read_to_string;

fn get_data(path: &str) -> HashMap<(u16, u16), Tile> {
    let raw = read_to_string(path).expect("problem with file");
    raw.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices().flat_map(move |(col, c)| {
                let pos = (row as u16, col as u16);
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

fn main_dijkstra(data: HashMap<(u16, u16), Tile>) {
    const MAX_DIST: u16 = u16::max_value();
    let source = State::from_hashmap(data);
    let src_hash = source.hash();

    let mut frontier = VecDeque::new();
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();

    frontier.push_front(source);
    distances.insert(src_hash, 0);

    while let Some(state) = frontier.pop_front() {
        let parent_hash = state.hash();
        visited.insert(parent_hash.clone());
        let &parent_dist = distances.get(&parent_hash).unwrap();

        for (next_state, rel_dist) in state.expand() {
            let hash = next_state.hash();
            if visited.contains(&hash) {
                continue;
            }
            let alt = parent_dist + rel_dist;
            if &alt < distances.get(&hash).unwrap_or(&MAX_DIST) {
                distances.insert(hash, alt);
                if !frontier.contains(&next_state) {
                    frontier.push_back(next_state);
                }
            }
        }
    }
    let min = distances
        .into_iter()
        .filter_map(
            |((hash, _pos), v)| {
                if hash.is_empty() {
                    Some(v)
                } else {
                    None
                }
            },
        )
        .min();
    println!("min distance found was {:?}", min);
}

fn main() {
    let mut args = env::args();
    let path: String = args.nth(1).expect("no data path provided");
    let data = get_data(&path);
    main_dijkstra(data);
}
