use std::collections::HashSet;
mod display;
mod evolve;
mod parse;

pub type Code = [bool; 512];

#[derive(Debug)]
pub struct Points {
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
    points: HashSet<(isize, isize)>,
    horizon_lit: bool,
}

impl std::ops::Deref for Points {
    type Target = HashSet<(isize, isize)>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

impl std::ops::DerefMut for Points {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}

pub type Error = &'static str;

fn main() {
    // let input = std::fs::read_to_string("data/input").unwrap();
    // let input = std::fs::read_to_string("data/example2").unwrap();
    // let input = std::fs::read_to_string("data/chris").unwrap();
    let input = std::fs::read_to_string("data/empty").unwrap();
    let (code, mut points) = parse::parse(&input).unwrap();
    println!("Quedaron: {}:\n{}", points.len(), points);
    points.evolve(&code);
    println!("Quedaron: {}:\n{}", points.len(), points);
    points.evolve(&code);
    println!("Quedaron: {}:\n{}", points.len(), points);
}
