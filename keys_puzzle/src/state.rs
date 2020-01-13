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
    meepos: Vec<(u16, u16)>,
    ///for each reachable key, how far is it and which can take it
    key_dists: HashMap<(u16, u16), (u16, usize)>,
}

impl State {
    pub fn from_hashmap(mut tiles: HashMap<(u16, u16), Tile>) -> Self {
        let meepos = tiles
            .iter()
            .filter_map(|(&pos, &t)| if t == Me { Some(pos) } else { None })
            .collect::<Vec<_>>();
        for m in &meepos {
            tiles.remove(&m);
        }
        State::new(tiles, meepos)
    }

    fn new(tiles: HashMap<(u16, u16), Tile>, meepos: Vec<(u16, u16)>) -> Self {
        let mut state = State {
            tiles,
            meepos,
            key_dists: HashMap::new(),
        };
        state.calc_distances();
        state
    }

    fn calc_distances(&mut self) {
        const NO_DISTANCE: &(u16, usize) = &(u16::max_value(), usize::max_value());
        let mut frontier = VecDeque::new();
        let mut distances: HashMap<(u16, u16), (u16, usize)> = HashMap::new();
        let mut visited = HashSet::new();
        for (meepo, me) in self.meepos.iter().enumerate() {
            frontier.push_front(*me);
            distances.insert(*me, (0, meepo));
        }

        while let Some((x, y)) = frontier.pop_front() {
            // mark as visited
            visited.insert((x, y));
            // expand in four directions checking which are valid
            for opt in &[(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)] {
                if let Some(kind) = self.tiles.get(opt) {
                    if visited.contains(opt) {
                        continue;
                    }
                    let &(parent_dist, parent_meepo) = distances.get(&(x, y)).unwrap();
                    match kind {
                        Door(_) => {
                            visited.insert(*opt);
                        }
                        Empty => {
                            let alt = parent_dist + 1;
                            let (cureent_dist, _) = distances.get(opt).unwrap_or(NO_DISTANCE);
                            if &alt < cureent_dist {
                                distances.insert(*opt, (alt, parent_meepo));
                                if !frontier.contains(opt) {
                                    frontier.push_back(*opt);
                                }
                            }
                        }
                        Key(_) => {
                            let alt = parent_dist + 1;
                            let (cureent_dist, _) = distances.get(opt).unwrap_or(NO_DISTANCE);
                            if &alt < cureent_dist {
                                // keys are not collected => can't be expanded
                                distances.insert(*opt, (alt, parent_meepo));
                                visited.insert(*opt);
                            }
                        }
                        Me => unreachable!(),
                    }
                }
            }
        }
        // leave only distances corresponding to keys
        self.key_dists = distances
            .into_iter()
            .filter(|(pos, _d)| {
                if self.meepos.contains(&pos) {
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

    pub fn hash(&self) -> (String, Vec<(u16, u16)>) {
        let mut remaining_keys: Vec<_> = self
            .key_dists
            .iter()
            .map(|(pos, _)| self.tiles.get(pos).unwrap().to_string())
            .collect();
        remaining_keys.sort_unstable();
        (
            remaining_keys.into_iter().collect::<String>(),
            self.meepos.clone(),
        )
    }

    /// Expand generates all reachable states from this point, with the
    /// distances to each one of them
    pub fn expand(&self) -> Vec<(State, u16)> {
        self.key_dists
            .iter()
            .map(|(&key_pos, &(dist, meepo))| {
                let mut new_tiles = self.tiles.clone();
                new_tiles.insert(key_pos, Me);
                for (i, meepo_pos) in self.meepos.iter().enumerate() {
                    if i == meepo {
                        new_tiles.insert(self.meepos[meepo], Empty);
                    } else {
                        new_tiles.insert(*meepo_pos, Me);
                    }
                }
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
                    if self.meepos.contains(&(y, x)) {
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
            "Me", self.meepos, "Distances", self.key_dists, "Tiles", self.tiles
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
