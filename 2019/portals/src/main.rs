mod maze;

use crate::maze::*;
use dijkstra::Explorer;
use std::env;

const Z_PORTAL: Tile = Portal(['Z', 'Z'], true);

type Position = (u16, u16);
type Level = u16;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct State {
    position: Position,
    level: Level,
}

fn expand(state: &State, maze: &Maze) -> Vec<(State, usize)> {
    let me = state.position;
    let lvl = state.level;
    let (x, y) = me;

    let mut branches = vec![];
    for &opt in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        // check that it is a valid movement
        if let Some(tile) = maze.get(&opt) {
            // now we need to find out where we end up and how many steps we moved
            // v2: find which recursion level we get
            let (real, steps) = match tile {
                Portal(auxp, is_outer) => {
                    if tile == &Z_PORTAL && lvl == 0 {
                        // supose we can stand on the portal like floor, but free
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
            branches.push((
                State {
                    position: real.0,
                    level: real.1,
                },
                steps,
            ));
        }
    }
    branches
}

fn main() {
    let mut args = env::args();
    let path: String = args.nth(1).expect("no data path provided");
    let (maze, start) = get_maze(&path);
    paint_maze(&maze, start);
    let source = State {
        position: start,
        level: 0,
    };

    let zz_position = maze
        .iter()
        .find(|(_k, &v)| v == Z_PORTAL)
        .expect("No ZZ in this map!")
        .clone()
        .0;

    let expand_state = |x: &State| expand(x, &maze);
    let is_end_state = move |x: &State| zz_position == &x.position && x.level == 0;

    let mut explorer = Explorer::new(source, expand_state, is_end_state);
    while explorer.step() {}
    let min = explorer.get_min_distance();
    println!("Min found was {:?}", min);
}
