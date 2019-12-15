use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn parse_data(lines: std::str::Lines) -> Vec<(usize, usize)> {
    lines
        .enumerate()
        .flat_map(|(column, line)| {
            line.char_indices()
                .flat_map(move |(row, c)| if c == '#' { Some((row, column)) } else { None })
        })
        .collect()
}

fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x.abs()
    } else {
        gcd(y, x.rem_euclid(y))
    }
}

/// gives the vector between as0 ans as1 such that its 'y' component is always
/// positive and its components are an ireducible fraction, if returns true if
/// the scalar to which one must multiply to get the segment from as0 to as1 is
/// positive, and and false otherwise
fn get_line(as0: &(usize, usize), as1: &(usize, usize)) -> (i32, i32) {
    let delta_y = as1.1 as i32 - as0.1 as i32;
    let delta_x = as1.0 as i32 - as0.0 as i32;
    let gcd = gcd(delta_y, delta_x);
    (delta_x / gcd, delta_y / gcd)
}

pub fn solve(path: &str) -> Option<((usize, usize), i32)> {
    let asteroids = parse_data(read_to_string(path).expect("bad input").lines());
    let vision_vectors = asteroids
        .iter()
        .cartesian_product(asteroids.iter())
        .filter_map(|(as0, as1)| {
            if as0 == as1 {
                None?
            }
            Some((*as0, get_line(as0, as1)))
        })
        .collect::<HashSet<_>>();
    let mut counts = HashMap::new();
    for (ast, vec) in vision_vectors {
        *counts.entry(ast).or_insert(0) += 1;
    }
    counts.clone().into_iter().max_by_key(|(_, count)| *count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
