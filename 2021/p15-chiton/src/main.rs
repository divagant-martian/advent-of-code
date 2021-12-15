use sqr::Sqr;

impl<const N: usize> Cave<N> {
    pub fn least_risky_path(&self) -> RiskLevel {
        #[derive(Default, Clone, Copy)]
        struct PosInfo {
            best_known_risk: Option<RiskLevel>,
            visited: bool,
        }

        let mut node_info: Sqr<PosInfo, N> = Sqr::default();
        node_info[(0, 0)].best_known_risk = Some(0);

        while let Some((pos, info)) = {
            node_info
                .iter()
                .filter(|(_pos, info)| !info.visited)
                .min_by_key(|(_pos, risk)| risk.best_known_risk.unwrap_or(u32::max_value()))
        } {
            // Must have a known best path
            let my_risk = info.best_known_risk.unwrap();

            if pos == (N - 1, N - 1) {
                return my_risk;
            }

            for n in Cave::<N>::neighbors(&pos, 1) {
                if !node_info[n].visited {
                    let risk_using_self = my_risk + self[n];
                    node_info[n].best_known_risk = node_info[n]
                        .best_known_risk
                        .map(|risk| risk.min(risk_using_self))
                        .or(Some(risk_using_self));
                }
            }
            node_info[pos].visited = true;
        }

        panic!("Unreachable end node")
    }

    pub fn neighbors(
        pos: &(usize, usize),
        tiles: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (y, x) = *pos;
        [
            (0, 1), /* (-1, 0) up    */
            (1, 2), /* (0, 1)  right */
            (2, 1), /* (1,0)   down  */
            (1, 0), /* (0,-1)  left  */
        ]
        .into_iter()
        .filter_map(move |(y_delta, x_delta)| {
            let ny = (y + y_delta).checked_sub(1)?;
            let nx = (x + x_delta).checked_sub(1)?;
            if ny < N * tiles && nx < N * tiles {
                Some((ny, nx))
            } else {
                None
            }
        })
    }
}

mod sqr;

type RiskLevel = u32;

type Cave<const N: usize> = Sqr<RiskLevel, N>;

type Error = &'static str;

impl<const N: usize> std::str::FromStr for Cave<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let risk_level = s
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).ok_or("Could not parse digit"))
                    .collect::<Result<Vec<_>, Error>>()?
                    .try_into()
                    .map_err(|_| "Wrong number of columns")
            })
            .collect::<Result<Vec<[_; N]>, Error>>()?
            .try_into()
            .map_err(|_| "Wrong number of lines")?;
        Ok(Sqr::new(risk_level))
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input").expect("Input file is present");
    let cave: Cave<100> = input.parse().expect("Input is ok");
    assert_eq!(cave.least_risky_path(), 619);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
        0163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138520
        2311944582
    ";

    const TEST_CAVE: Cave<10> = Sqr::new([
        [0, 1, 6, 3, 7, 5, 1, 7, 4, 2],
        [1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
        [2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
        [3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
        [7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
        [1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
        [1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
        [3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
        [1, 2, 9, 3, 1, 3, 8, 5, 2, 0],
        [2, 3, 1, 1, 9, 4, 4, 5, 8, 2],
    ]);

    #[test]
    fn test_parse() {
        assert_eq!(TEST_INPUT.parse::<Cave<10>>().unwrap(), TEST_CAVE)
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(
            Cave::<10>::neighbors(&(0, 0), 1).collect::<Vec<_>>(),
            vec![(0, 1), (1, 0)]
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(TEST_CAVE.least_risky_path(), 40);
    }
}
