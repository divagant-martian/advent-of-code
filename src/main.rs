mod maze;

use crate::maze::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::io::Read;

const Z_PORTAL: Tile = Portal(['Z', 'Z'], true);

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

    // where am I in the map and in which recursion level
    frontier.push_front((start, 0));
    distances.insert((start, 0), 0);

    while let Some((me, lvl)) = frontier.pop_front() {
        let (x, y) = me;
        visited.insert((me, lvl));
        let &my_dist = distances.get(&(me, lvl)).unwrap();

        // just for debug purposes....
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
        if target == me && lvl == 0 {
            break;
        }
        for &opt in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            // check that it is a valid movement
            if let Some(tile) = maze.get(&opt) {
                // now we need to find out where we end up and how many steps we moved
                // v2: find which recursion level we get
                let (real, steps) = match tile {
                    Portal(auxp, is_outer) => {
                        if tile == &Z_PORTAL && lvl == 0 {
                            // supose we can stand on the portal like floor, but free
                            target = opt;
                            ((opt, lvl), 0)
                        } else if let Some((&other, _)) =
                            maze.iter().find(|(_, &v)| v == Portal(*auxp, !is_outer))
                        {
                            let is_outer = *is_outer;
                            // find the corresponding portal to this one, if any
                            // entering a portal doesn't add steps
                            if lvl == 0 {
                                // only inner portals work on lvl 0
                                if is_outer {
                                    ((me, lvl), 0)
                                } else {
                                    // is inner
                                    ((other, lvl + 1), 0)
                                }
                            } else if is_outer {
                                ((other, lvl - 1), 0)
                            } else {
                                ((other, lvl + 1), 0)
                            }
                        } else {
                            // can't enter the portal
                            ((me, lvl), 0)
                        }
                    }
                    Floor => ((opt, lvl), 1),
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
    println!("{:?}", distances.get(&(target, 0)));
}
