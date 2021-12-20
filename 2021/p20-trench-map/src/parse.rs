use std::{collections::HashSet, str::FromStr};

use crate::{Code, Error, Points};

pub fn parse(input: &str) -> Result<(Code, Points), Error> {
    let (code, points) = input
        .trim()
        .split_once("\n\n")
        .ok_or("Missing points in input")?;
    let code: Code = code
        .trim()
        .chars()
        .map(|c| match c {
            '.' => Ok(false),
            '#' => Ok(true),
            _ => Err("Unexpected character in the image enhancement code."),
        })
        .collect::<Result<Vec<_>, &'static str>>()?
        .try_into()
        .map_err(|_| "Wrong number of mappings in image inhancement code.")?;

    let points = Points::from_str(points)?;

    Ok((code, points))
}

impl FromStr for Points {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut min_x = isize::max_value();
        let mut min_y = isize::max_value();
        let mut max_x = 0isize;
        let mut max_y = 0isize;
        let mut points = HashSet::default();
        for (y, line) in s.trim().lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    let y = y as isize;
                    let x = x as isize;
                    max_y = max_y.max(y);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    min_x = min_x.min(x);

                    points.insert((y, x));
                }
            }
        }
        Ok(Points {
            points,
            min_x,
            min_y,
            max_x,
            max_y,
            horizon_lit: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
            ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

            #..#.
            #....
            ##..#
            ..#..
            ..###
        ";
        let (code, points) = parse(input).unwrap();
        assert_eq!(code[0], false);
        assert_eq!(points.len(), 10);
    }
}
