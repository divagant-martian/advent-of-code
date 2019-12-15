use itertools::Itertools;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::f64::consts::PI;
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
fn get_line(as0: &(usize, usize), as1: &(usize, usize)) -> ((i32, i32), i32) {
    let delta_y = as1.1 as i32 - as0.1 as i32;
    let delta_x = as1.0 as i32 - as0.0 as i32;
    let gcd = gcd(delta_y, delta_x);
    (
        (delta_x / gcd, delta_y / gcd),
        delta_x.pow(2) + delta_y.pow(2),
    )
}

pub fn solve_ten_a(path: &str) -> Option<((usize, usize), i32)> {
    let asteroids = parse_data(read_to_string(path).expect("bad input").lines());
    let vision_vectors = asteroids
        .iter()
        .cartesian_product(asteroids.iter())
        .filter_map(|(as0, as1)| {
            if as0 == as1 {
                None?
            }
            let (vec, _) = get_line(as0, as1);
            Some((*as0, vec))
        })
        .collect::<HashSet<_>>();
    let mut counts = HashMap::new();
    for (ast, _) in vision_vectors {
        *counts.entry(ast).or_insert(0) += 1;
    }
    counts.clone().into_iter().max_by_key(|(_, count)| *count)
}

#[derive(Debug)]
pub struct Angle(f64);

impl Angle {
    pub fn from_slope(x: f64, y: f64) -> Self {
        Angle(x.atan2(-y).rem_euclid(2.0 * PI))
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        let i_self = self.0;
        let i_other = other.0;
        (i_self == i_other) || (i_self.is_nan() && i_other.is_nan())
    }
}

impl Eq for Angle {}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        let i_self = &self.0;
        let i_other = &other.0;
        i_self.partial_cmp(i_other).unwrap_or_else(|| {
            if i_self.is_nan() {
                if i_other.is_nan() {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Less
            }
        })
    }
}

fn get_angle(vec: (i32, i32)) -> Angle {
    Angle::from_slope(vec.0 as f64, vec.1 as f64)
}

pub fn solve_ten_b(path: &str, as0: (usize, usize), nth: i32) -> (usize, usize) {
    let asteroids = parse_data(read_to_string(path).expect("bad input").lines());
    let vision_vectors = asteroids.iter().filter_map(|as1| {
        if &as0 == as1 {
            None?
        }
        let (vec, distance) = get_line(&as0, as1);
        Some((get_angle(vec), distance, as1))
    });
    let mut order = BTreeMap::new();
    for (angle, dist, ast) in vision_vectors {
        let ast_list = order.entry(angle).or_insert(vec![]);
        let index = ast_list.binary_search(&(-dist, ast)).unwrap_err();
        ast_list.insert(index, (-dist, ast));
    }

    let mut remaining = nth;
    for (_, vec) in order.iter_mut() {
        if let Some((_, ast)) = vec.pop() {
            remaining -= 1;
            if remaining == 0 {
                return *ast;
            }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
