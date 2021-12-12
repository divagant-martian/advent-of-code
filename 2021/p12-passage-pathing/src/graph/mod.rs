use self::{cave::Cave, path_finder::PathFinder};

mod cave;
mod parse;
mod path_finder;

pub type CaveId = usize;

#[derive(Debug)]
pub struct CaveSystem<'a> {
    /// List of caves.
    caves: Vec<Cave<'a>>,
    /// Bi-directional connections from a to b, where a and b and indeces of `caves`.
    connections: Vec<(CaveId, CaveId)>,
}

impl<'a> CaveSystem<'a> {
    pub fn iter(&'a self) -> PathFinder<'a> {
        PathFinder::new(self)
    }

    pub fn index_of(&self, cave: &Cave) -> Option<CaveId> {
        self.caves.iter().position(|c| c == cave)
    }

    pub fn connections(&self, idx: CaveId) -> impl Iterator<Item = CaveId> + '_ {
        self.connections.iter().filter_map(move |&(a, b)| {
            if a == idx {
                Some(b)
            } else if b == idx {
                Some(a)
            } else {
                None
            }
        })
    }

    #[allow(dead_code)]
    pub fn format_path(&self, path: &[CaveId]) -> String {
        let mut repr: String = path
            .iter()
            .map(|id| format!("{}-", self[*id].as_str()))
            .collect();
        repr.pop();
        repr
    }
}

impl<'a> std::ops::Index<CaveId> for CaveSystem<'a> {
    type Output = Cave<'a>;

    fn index(&self, index: CaveId) -> &Self::Output {
        &self.caves[index]
    }
}
