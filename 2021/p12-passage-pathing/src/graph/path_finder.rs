use super::{cave::Cave, CaveId, CaveSystem};

pub struct PathFinder<'a> {
    cave_system: &'a CaveSystem<'a>,
    current_path: Vec<CaveId>,
    queue: Vec<(CaveId, usize)>,
    enough_time: bool,
}

impl<'a> PathFinder<'a> {
    pub fn new(cave_system: &'a CaveSystem) -> PathFinder<'a> {
        let start_index = cave_system
            .index_of(&Cave::Start)
            .expect("start is always present");
        PathFinder::<'a> {
            cave_system,
            current_path: Vec::default(),
            queue: vec![(start_index, 0)],
            enough_time: false,
        }
    }

    pub fn with_enough_time(mut self) -> Self {
        self.enough_time = true;
        self
    }
}

impl<'a> Iterator for PathFinder<'a> {
    type Item = Vec<CaveId>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((to_push, depth)) = self.queue.pop() {
            self.current_path.truncate(depth);
            self.current_path.push(to_push);
            // queue the rest
            let children_depth = self.current_path.len();
            for son in self.cave_system.connections(to_push) {
                if self.enough_time {
                    let has_duplicated_small_caves = {
                        let mut seen =
                            std::collections::HashSet::with_capacity(self.current_path.len());
                        let mut duplicate = false;
                        for i in &self.current_path {
                            if self.cave_system[*i].is_small() && !seen.insert(*i) {
                                duplicate = true;
                                break;
                            }
                        }
                        duplicate
                    };

                    let son_cave = &self.cave_system[son];
                    if !self.current_path.contains(&son)
                        || son_cave.is_big()
                        || (son_cave.is_small() && !has_duplicated_small_caves)
                    {
                        self.queue.push((son, children_depth));
                    }
                } else if !self.current_path.contains(&son) || self.cave_system[son].is_big() {
                    self.queue.push((son, children_depth));
                }
            }

            // check this path
            if let Cave::End = &self.cave_system[to_push] {
                return Some(self.current_path.clone());
            }
        }
        None
    }
}
