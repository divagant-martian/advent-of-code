use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("data/input.txt").expect("input is ok");
    let grid = parse_grid(&input);
    let lows_pos = low_positions(&grid);

    let lows = lows_pos.iter().map(|&(i, j)| grid[i][j]).collect();
    println!("Risk: {}", risk(&lows));

    let mut basin_lens = lows_pos
        .iter()
        .map(|&low| Basin::from_low(low, &grid).members.len())
        .collect::<Vec<_>>();
    basin_lens.sort_unstable();
    let ans = basin_lens.iter().rev().take(3).product::<usize>();
    println!("{}", ans);
}

type Grid = Vec<Vec<u16>>;

pub fn parse_grid(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .expect("digit is ok")
                        .try_into()
                        .expect("just one digit")
                })
                .collect()
        })
        .collect::<Vec<_>>()
}

struct Basin {
    start_point: (usize, usize),
    members: Vec<(usize, usize)>,
}

impl Basin {
    pub fn from_low(low: (usize, usize), grid: &Grid) -> Self {
        let mut members = vec![low];
        let mut to_visit = VecDeque::from(vec![low]);
        while let Some((x, y)) = to_visit.pop_front() {
            for point in [
                (x + 1, y),               // right
                (x.saturating_sub(1), y), // left
                (x, y + 1),               // up
                (x, y.saturating_sub(1)), // down
            ] {
                if !members.contains(&point) {
                    if let Some(&n) = grid.get(point.0).and_then(|row| row.get(point.1)) {
                        if n != 9 && n > grid[x][y] {
                            members.push(point);
                            to_visit.push_back(point);
                        }
                    }
                }
            }
        }

        Basin {
            start_point: low,
            members,
        }
    }
}

pub fn low_positions(grid: &Grid) -> Vec<(usize, usize)> {
    let mut lows = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, &n) in line.iter().enumerate() {
            let mut is_low_point = true;
            if i > 0 {
                // up
                is_low_point &= n < grid[i - 1][j];
            }
            if j > 0 {
                // left
                is_low_point &= n < line[j - 1];
            }
            if let Some(line) = grid.get(i + 1) {
                // down
                is_low_point &= n < line[j];
            }
            if let Some(&m) = line.get(j + 1) {
                // right
                is_low_point &= n < m;
            }

            if is_low_point {
                lows.push((i, j));
            }
        }
    }
    lows
}

pub fn risk(lows: &Vec<u16>) -> u16 {
    lows.iter().map(|n| n + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let grid = parse_grid(
            "
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ",
        );

        let lows_pos = low_positions(&grid);

        let lows = lows_pos.iter().map(|&(i, j)| grid[i][j]).collect();
        assert_eq!(lows, vec![1, 0, 5, 5]);
        assert_eq!(risk(&lows), 15);

        let le_low = lows_pos[2];
        println!("Le low {:?}", le_low);
        let basin = Basin::from_low(le_low, &grid);
        assert_eq!(basin.members.len(), 14);

        let mut basin_lens = lows_pos
            .iter()
            .map(|&low| Basin::from_low(low, &grid).members.len())
            .collect::<Vec<_>>();
        basin_lens.sort_unstable();
        let ans = basin_lens.iter().rev().take(3).product::<usize>();
        assert_eq!(ans, 1134);
    }
}
