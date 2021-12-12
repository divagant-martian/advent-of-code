use super::{cave::Cave, CaveSystem};

impl<'a, 'b: 'a> std::convert::TryFrom<&'b str> for CaveSystem<'a> {
    type Error = &'static str;

    fn try_from(value: &'b str) -> Result<Self, Self::Error> {
        let mut caves = Vec::default();
        let mut connections = Vec::default();

        for line in value
            .trim()
            .lines()
            .map(|l| l.split_once('-').ok_or("wrong format"))
        {
            let (node_a, node_b) = line?;

            // insert the nodes if not present
            let i_a = {
                let node_a: Cave = node_a.try_into()?;
                if let Some(i) = caves.iter().position(|c| c == &node_a) {
                    i
                } else {
                    let pos = caves.len();
                    caves.insert(pos, node_a);
                    pos
                }
            };

            let i_b = {
                let node_b: Cave = node_b.try_into()?;
                if let Some(i) = caves.iter().position(|c| c == &node_b) {
                    i
                } else {
                    let pos = caves.len();
                    caves.insert(pos, node_b);
                    pos
                }
            };

            connections.push((i_a, i_b));
        }

        Ok(CaveSystem { caves, connections })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input: Result<CaveSystem, _> = "
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "
        .try_into();
        assert!(input.is_ok())
    }
}
