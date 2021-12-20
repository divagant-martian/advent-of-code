#![allow(warnings)]

use std::collections::BTreeSet;

mod matches;
mod ops;
mod parse;
mod transform;

pub type Error = &'static str;
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Default)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }

    pub fn norm(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scanner {
    origin: Point,
    beacons: BTreeSet<Point>,
}

impl Scanner {
    pub fn len(&self) -> usize {
        self.beacons.len()
    }

    pub(crate) fn origin(&self) -> Point {
        self.origin.clone()
    }
}
