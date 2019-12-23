mod maze;

use crate::maze::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::io::Read;

const Z_PORTAL: Tile = Portal(['Z', 'Z']);
fn main() {
    let mut args = env::args();
    let path: String = args.nth(1).expect("no data path provided");
    let (tiles, start) = get_maze(&path);
    paint_maze(&tiles, start);
    dijkstra(tiles, start);
}

fn dijkstra(maze: Maze, start: (u16, u16)) {
    const MAX_DIST: u16 = u16::max_value();

    let mut frontier = VecDeque::new();
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    let mut target = (MAX_DIST, MAX_DIST);

    frontier.push_front(start);
    distances.insert(start, 0);

    while let Some(me) = frontier.pop_front() {
        let (x, y) = me;
        visited.insert(me);
        let &my_dist = distances.get(&me).unwrap();
        if false {
            println!("visiting with distance {}", my_dist);
            paint_maze(&maze, me);
            println!("\n");

            let mut b = [0];
            let mut stdin = std::io::stdin();
            stdin.read(&mut b).unwrap();

            match b[0] {
                b'q' => return,
                _ => (),
            }
        }
        // ending condition
        if target == me {
            break;
        }
        for &opt in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            // check that it is a valid movement
            if let Some(tile) = maze.get(&opt) {
                // now we need to find out where we end up and how many steps we moved
                let (real, steps) = match tile {
                    Portal([_, _]) => {
                        if tile == &Z_PORTAL {
                            // supose we can stand on the portal like floor
                            target = opt;
                            (opt, 0)
                        } else if let Some((&other, _)) =
                            maze.iter().find(|(&k, &v)| k != opt && v == *tile)
                        {
                            // find the corresponding portal to this one, if any
                            // entering a portal doesn't add steps
                            (other, 0)
                        } else {
                            // can't enter the portal
                            (me, 0)
                        }
                    }
                    Floor => (opt, 1),
                };

                let alt_dist = my_dist + steps;
                if &alt_dist < distances.get(&real).unwrap_or(&MAX_DIST) {
                    distances.insert(real, alt_dist);
                    if !frontier.contains(&real) {
                        frontier.push_back(real);
                    }
                }
            }
        }
    }
    println!("{:?}", distances.get(&target));
}
