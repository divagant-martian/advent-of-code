use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub type Dist = usize;
const MAX_DIST: Dist = Dist::max_value();
pub struct Explorer<
    Node: Eq + Hash + Clone,
    IterN: IntoIterator<Item = (Node, usize)>,
    ExpandFn: Fn(&Node) -> IterN,
    TestFinalFn: Fn(&Node) -> bool,
> {
    visited: HashSet<Node>,
    distances: HashMap<Node, Dist>,
    frontier: VecDeque<Node>,
    predecesors: HashMap<Node, Node>,
    expand: ExpandFn,
    is_final: TestFinalFn,
}

impl<
        Node: Eq + Hash + Clone,
        IterN: IntoIterator<Item = (Node, usize)>,
        ExpandFn: Fn(&Node) -> IterN,
        TestFinalFn: Fn(&Node) -> bool,
    > Explorer<Node, IterN, ExpandFn, TestFinalFn>
{
    pub fn new(source: Node, expand: ExpandFn, is_final: TestFinalFn) -> Self {
        let mut frontier = VecDeque::new();
        let mut distances = HashMap::new();
        distances.insert(source.clone(), 0);
        frontier.push_front(source);
        Explorer {
            visited: HashSet::new(),
            distances: distances,
            frontier: frontier,
            predecesors: HashMap::new(),
            expand,
            is_final,
        }
    }

    pub fn step(&mut self) -> bool {
        if let Some(state) = self.frontier.pop_front() {
            self.visited.insert(state.clone());
            if (self.is_final)(&state) {
                println!("found final");
                return false;
            }

            let &parent_dist = self.distances.get(&state).unwrap();
            for (next_state, rel_dist) in (self.expand)(&state) {
                if self.visited.contains(&next_state) {
                    continue;
                }
                let alt = parent_dist + rel_dist;
                if &alt < self.distances.get(&next_state).unwrap_or(&MAX_DIST) {
                    self.distances.insert(next_state.clone(), alt);
                    if !self.frontier.contains(&next_state) {
                        self.frontier.push_back(next_state.clone());
                        // self.predecesors.insert(next_state, state.clone());
                    }
                }
            }
            return true;
        }
        false
    }

    pub fn get_distance(&self, state: &Node) -> Option<Dist> {
        self.distances.get(state).cloned()
    }

    pub fn get_min_distance(&self) -> Option<Dist> {
        self.distances
            .iter()
            .filter_map(|(k, v)| if (self.is_final)(k) { Some(v) } else { None })
            .min()
            .cloned()
    }

    pub fn get_path(&self, node: &Node) -> Vec<Node> {
        let mut ans = vec![];
        let mut state = node;
        ans.push(node.clone());
        while let Some(pre) = self.predecesors.get(state) {
            ans.push(pre.clone());
            state = pre;
        }
        ans
    }
}
