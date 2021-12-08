use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("data/input.txt").expect("Input is present");
    let segments = parse_input(&input);
    println!("{}", points_on_straight_intersecting_lines(&segments).len());
    println!("{}", intersecting_points(&segments).len())
}

fn points_on_straight_intersecting_lines(segments: &Vec<Segment>) -> HashMap<Point, usize> {
    let mut points = HashMap::default();
    let straigh_lines: Vec<_> = segments.iter().filter(|s| s.is_straight()).collect();
    let n = straigh_lines.len();
    for i in 0..n {
        for j in i + 1..n {
            let intersections = straigh_lines[i].intersections(straigh_lines[j]);
            for p in intersections {
                *points.entry(p).or_default() += 1;
            }
        }
    }
    points
}

fn intersecting_points(segments: &Vec<Segment>) -> HashMap<Point, usize> {
    let mut points = HashMap::default();
    let n = segments.len();
    for i in 0..n {
        for j in i + 1..n {
            let intersections = segments[i].intersections(&segments[j]);
            for p in intersections {
                *points.entry(p).or_default() += 1;
            }
        }
    }
    points
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl std::convert::From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone)]
struct Segment {
    init: Point,
    m: Point,
    max_t: u16,
}

struct SegmentIter {
    max_t: i32,
    t: i32,
    init: Point,
    m: Point,
}
impl std::iter::Iterator for SegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.t <= self.max_t {
            let p = Point {
                x: self.init.x + self.t * self.m.x,
                y: self.init.y + self.t * self.m.y,
            };
            self.t += 1;
            Some(p)
        } else {
            None
        }
    }
}

impl Segment {
    fn is_vertical(&self) -> bool {
        self.m.x == 0
    }

    fn is_horizontal(&self) -> bool {
        self.m.y == 0
    }

    fn is_straight(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    fn iter(&self) -> SegmentIter {
        SegmentIter {
            max_t: self.max_t as i32,
            t: 0,
            init: self.init,
            m: self.m,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        match (self.m.x.abs(), self.m.y.abs()) {
            (0, 0) => unreachable!("sadfja"),
            (0, _) => {
                // vertical
                let tx = point.x - self.init.x;
                let ty = (point.y - self.init.y) / self.m.y;

                tx == 0 && ty >= 0 && ty <= self.max_t.into()
            }
            (_, 0) => {
                // horizontal
                let ty = point.y - self.init.y;
                let tx = (point.x - self.init.x) / self.m.x;

                0 == ty && tx >= 0 && tx <= self.max_t.into()
            }
            _ => {
                let tym = point.y - self.init.y;
                let txm = point.x - self.init.x;

                if tym % self.m.y != 0 || txm % self.m.x != 0 {
                    return false;
                }

                let tx = txm / self.m.x;
                let ty = tym / self.m.y;
                tx == ty && tx >= 0 && tx <= self.max_t.into()
            }
        }
    }

    fn intersections(&self, other: &Segment) -> Vec<Point> {
        other.iter().filter(|p| self.contains(&p)).collect()
    }

    fn end(&self) -> Point {
        let t = self.max_t as i32;
        Point {
            x: self.init.x + self.m.x * t,
            y: self.init.y + self.m.y * t,
        }
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let end = self.end();
        f.write_fmt(format_args!(
            "{},{} -> {},{}",
            self.init.x, self.init.y, end.x, end.y
        ))
    }
}

fn parse_input(input: &str) -> Vec<Segment> {
    input
        .trim()
        .lines()
        .map(str::parse::<Segment>)
        .collect::<Result<_, _>>()
        .expect("Input is ok")
}

impl std::str::FromStr for Segment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s
            .trim()
            .split(" -> ")
            .flat_map(|l| l.split(',').map(str::parse::<u16>))
            .collect::<Result<_, _>>()
            .map_err(|_| "Could not parse input.")?;

        let (x0, y0, x1, y1) = (
            parts[0] as i32,
            parts[1] as i32,
            parts[2] as i32,
            parts[3] as i32,
        );
        let up = y1 - y0;
        let down = x1 - x0;
        let gcd = gcd(up.abs(), down.abs());
        Ok(Segment {
            init: Point { x: x0, y: y0 },
            m: Point {
                x: down / gcd,
                y: up / gcd,
            },
            max_t: gcd as u16,
        })
    }
}
// Euclid's two-thousand-year-old algorithm for finding the greatest common divisor.
fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end() {
        let segment: Segment = str::parse("0,9 -> 5,9").expect("format is ok");
        assert_eq!(segment.init, Point { x: 0, y: 9 });
        assert_eq!(segment.end(), Point { x: 5, y: 9 });
    }

    #[test]
    fn test_points_horizontal() {
        let segment: Segment = str::parse("0,9 -> 5,9").expect("format is ok");
        let expected_points = vec![
            (0, 9).into(),
            (1, 9).into(),
            (2, 9).into(),
            (3, 9).into(),
            (4, 9).into(),
            (5, 9).into(),
        ];
        assert_eq!(segment.iter().collect::<Vec<Point>>(), expected_points);
    }

    #[test]
    fn test_points_vertical() {
        let segment: Segment = str::parse("1,2 -> 1,4").expect("format is ok");
        let expected_points = vec![(1, 2).into(), (1, 3).into(), (1, 4).into()];
        assert_eq!(segment.iter().collect::<Vec<Point>>(), expected_points);
    }

    #[test]
    fn test_points() {
        let segment: Segment = str::parse("0,0 -> 4,6").expect("format is ok");
        let expected_points = vec![(0, 0).into(), (2, 3).into(), (4, 6).into()];
        assert_eq!(segment.iter().collect::<Vec<Point>>(), expected_points);
    }

    #[test]
    fn intersections() {
        // same line
        let segment_a: Segment = str::parse("0,0 -> 4,6").expect("format is ok");
        let segment_b = segment_a.clone();
        assert_eq!(
            segment_a.intersections(&segment_b),
            segment_a.iter().collect::<Vec<Point>>()
        );

        // contained b in a
        let segment_a: Segment = str::parse("0,9 -> 5,9").expect("format is ok");
        let segment_b: Segment = str::parse("0,9 -> 2,9").expect("format is ok");
        assert_eq!(
            segment_a.intersections(&segment_b),
            segment_b.iter().collect::<Vec<Point>>()
        );

        // horizontal and vertial
        let segment_a: Segment = str::parse("0,9 -> 5,9").expect("format is ok");
        let segment_b: Segment = str::parse("3,0 -> 3,10").expect("format is ok");
        assert_eq!(segment_a.intersections(&segment_b), vec![(3, 9).into()]);

        // horizontal and diagonal
        let segment_a: Segment = str::parse("0,9 -> 15,9").expect("format is ok");
        let segment_b: Segment = str::parse("9,9 -> 7,7").expect("format is ok");
        assert_eq!(segment_a.intersections(&segment_b), vec![(9, 9).into()]);
    }

    #[test]
    fn example_part_1() {
        let input = "
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        ";
        let segments = parse_input(input);
        let intersections = points_on_straight_intersecting_lines(&segments);
        println!("{:?}", intersections);
        assert_eq!(intersections.len(), 5);
    }

    #[test]
    fn example_part_2() {
        let input = "
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        ";
        let segments = parse_input(input);
        let intersections = intersecting_points(&segments);
        println!("{:?}", intersections);
        assert_eq!(intersections.len(), 12);
    }
}
