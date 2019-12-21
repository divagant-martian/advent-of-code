use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
pub use Tile::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Key(char),
    Door(char),
    Empty,
    Me,
}

pub struct State {
    tiles: HashMap<(u16, u16), Tile>,
    me: (u16, u16),
    key_dists: HashMap<(u16, u16), u16>,
}

impl State {
    pub fn from_hashmap(mut tiles: HashMap<(u16, u16), Tile>) -> Self {
        let (&me, _) = tiles.iter().find(|(_, &t)| t == Me).expect("me not found");
        tiles.remove(&me);
        State::new(tiles, me)
    }

    pub fn new(tiles: HashMap<(u16, u16), Tile>, me: (u16, u16)) -> Self {
        if let Some((&me2, _)) = tiles.iter().find(|(_pos, &t)| t == Me) {
            if me2 == me {
                panic!("duplicated me");
            }
            panic!("inconsistent me position between tiles and me params");
        }

        let mut state = State {
            tiles,
            me,
            key_dists: HashMap::new(),
        };
        state.calc_distances();
        state
    }

    fn calc_distances(&mut self) {
        let mut frontier = VecDeque::new();
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let me = self.me;
        frontier.push_front(me);
        distances.insert(me, 0);

        while let Some((x, y)) = frontier.pop_front() {
            // mark as visited
            visited.insert((x, y));
            // expand in four directions checking which are valid
            for opt in &[(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
                if let Some(kind) = self.tiles.get(opt) {
                    if visited.contains(opt) {
                        continue;
                    }
                    match kind {
                        Door(_) => {
                            visited.insert(*opt);
                        }
                        Empty => {
                            let alt = distances.get(&(x, y)).unwrap() + 1;
                            if &alt < distances.get(opt).unwrap_or(&u16::max_value()) {
                                distances.insert(*opt, alt);
                                if !frontier.contains(opt) {
                                    frontier.push_back(*opt);
                                }
                            }
                        }
                        Key(_) => {
                            let alt = distances.get(&(x, y)).unwrap() + 1;
                            if &alt < distances.get(opt).unwrap_or(&u16::max_value()) {
                                // keys are not collected => can't be expanded
                                distances.insert(*opt, alt);
                                visited.insert(*opt);
                            }
                        }
                        Me => unreachable!(),
                    }
                }
            }
        }
        self.key_dists = distances
            .into_iter()
            .filter(|(pos, _d)| {
                if pos == &self.me {
                    return false;
                }
                if let Key(_) = self.tiles.get(&pos).expect("pos not found in retain") {
                    return true;
                } else {
                    return false;
                }
            })
            .collect();
    }

    pub fn hash(&self) -> (String, (u16, u16)) {
        let mut remaining_keys: Vec<_> = self
            .key_dists
            .iter()
            .map(|(pos, _)| self.tiles.get(pos).unwrap().to_string())
            .collect();
        remaining_keys.sort_unstable();
        (remaining_keys.into_iter().collect::<String>(), self.me)
    }

    /// Expand generates all reachable states from this point, with the
    /// distances to each one of them
    pub fn expand(&self) -> Vec<(State, u16)> {
        self.key_dists
            .iter()
            .map(|(&key_pos, &dist)| {
                let mut new_tiles = self.tiles.clone();
                new_tiles.insert(key_pos, Me);
                new_tiles.insert(self.me, Empty);
                if let Key(k) = self.tiles.get(&key_pos).unwrap() {
                    // open the corresponding door
                    if let Some((&door_pos, _)) = self
                        .tiles
                        .iter()
                        .find(|(_, &v)| v == Door(k.to_ascii_uppercase()))
                    {
                        // the last key has no visible door
                        // new_tile2s
                        new_tiles.insert(door_pos, Empty);
                    }
                }
                (State::from_hashmap(new_tiles), dist)
            })
            .collect()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (max_y, max_x) = self
            .tiles
            .iter()
            .fold((0, 0), |(max_y, max_x), ((y, x), _)| {
                (max_y.max(*y), max_x.max(*x))
            });
        let rep: String = (0..=max_y + 1)
            .map(|y| {
                (0..=max_x + 1).fold(String::new(), |acc, x| {
                    if (y, x) == self.me {
                        return acc + &Me.to_string();
                    }
                    if let Some(kind) = self.tiles.get(&(y, x)) {
                        return acc + &kind.to_string();
                    }
                    acc + "#"
                }) + "\n"
            })
            .collect();
        write!(f, "{}", rep)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "            State\n{:>11}:{:?}\n{:>11}:{:?}\n{:>11}:{:?}",
            "Me", self.me, "Distances", self.key_dists, "Tiles", self.tiles
        )
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rep = match self {
            Key(k) | Door(k) => k.to_string(),
            Empty => String::from("."),
            Me => String::from("@"),
        };
        write!(f, "{}", rep)
    }
}
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}
