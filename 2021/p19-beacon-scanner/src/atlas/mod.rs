pub type Error = &'static str;
#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Scanner {
    beacons: Vec<Point>,
}
#[allow(dead_code)]
const TRANSFORMATIONS: &[fn(Point) -> Point] = &[
    |p| p,
    |Point { x, y, z }| Point::new(x, -z, y),
    |Point { x, y, z }| Point::new(x, -y, -z),
    |Point { x, y, z }| Point::new(x, z, -y),
    |Point { x, y, z }| Point::new(-y, x, z),
    |Point { x, y, z }| Point::new(z, x, y),
    |Point { x, y, z }| Point::new(y, x, -z),
    |Point { x, y, z }| Point::new(-z, x, -y),
    |Point { x, y, z }| Point::new(-x, -y, z),
    |Point { x, y, z }| Point::new(-x, -z, -y),
    |Point { x, y, z }| Point::new(-x, y, -z),
    |Point { x, y, z }| Point::new(-x, z, y),
    |Point { x, y, z }| Point::new(y, -x, z),
    |Point { x, y, z }| Point::new(z, -x, -y),
    |Point { x, y, z }| Point::new(-y, -x, -z),
    |Point { x, y, z }| Point::new(-z, -x, y),
    |Point { x, y, z }| Point::new(-z, y, x),
    |Point { x, y, z }| Point::new(y, z, x),
    |Point { x, y, z }| Point::new(z, -y, x),
    |Point { x, y, z }| Point::new(-y, -z, x),
    |Point { x, y, z }| Point::new(-z, -y, -x),
    |Point { x, y, z }| Point::new(-y, z, -x),
    |Point { x, y, z }| Point::new(z, y, -x),
    |Point { x, y, z }| Point::new(y, -z, -x),
];

mod parse;
