use std::collections::HashSet;
use std::fs::read_to_string;

type Position = isize;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Cube {
    x: Position,
    y: Position,
    z: Position,
    w: Position,
}

impl Cube {
    fn neighbors(&self) -> Vec<Cube> {
        let mut neighbors = Vec::with_capacity(26);
        for x_offset in &[-1, 0, 1] {
            for y_offset in &[-1, 0, 1] {
                for z_offset in &[-1, 0, 1] {
                    for w_offset in &[-1, 0, 1] {
                        if *x_offset != 0_isize
                            || *y_offset != 0_isize
                            || *z_offset != 0_isize
                            || *w_offset != 0_isize
                        {
                            neighbors.push(Cube {
                                x: self.x + x_offset,
                                y: self.y + y_offset,
                                z: self.z + z_offset,
                                w: self.w + w_offset,
                            });
                        }
                    }
                }
            }
        }
        neighbors
    }
}

fn load_active_cubes(filename: &'static str) -> HashSet<Cube> {
    let mut cubes = HashSet::new();
    for (y, l) in read_to_string(filename)
        .expect("bad input")
        .lines()
        .enumerate()
    {
        for (x, c) in l.char_indices() {
            if c == '#' {
                cubes.insert(Cube {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                    w: 0,
                });
            }
        }
    }

    cubes
}

#[allow(unused)]
/*
fn paint(cubes: &HashSet<Cube>) {
    let ((max_x, max_y, max_z), (min_x, min_y, min_z)) = cubes.iter().fold(
        ((0, 0, 0), (0, 0, 0)),
        |((max_x, max_y, max_z), (min_x, min_y, min_z)), Cube { x, y, z }| {
            (
                (max_x.max(*x), max_y.max(*y), max_z.max(*z)),
                (min_x.min(*x), min_y.min(*y), min_z.min(*z)),
            )
        },
    );

    let mut row = String::new();
    for z in min_z..=max_z {
        println!(" [z = {}]", z);
        for y in min_y..=max_y {
            row.clear();
            row += "   ";
            for x in min_x..=max_x {
                let c = if cubes.contains(&Cube { x, y, z }) {
                    '#'
                } else {
                    '.'
                };
                row.push(c);
            }
            println!("{}", row);
        }
    }
    println!()
}*/

fn evolve(cubes: HashSet<Cube>) -> HashSet<Cube> {
    let mut new_cubes = HashSet::new();
    let mut to_check = HashSet::new();
    for active_cube in &cubes {
        let mut active_neighbors = 0;
        for n in active_cube.neighbors() {
            if cubes.contains(&n) {
                active_neighbors += 1;
            } else {
                to_check.insert(n);
            }
        }
        // active && 2 or 3 active => active
        if active_neighbors == 2 || active_neighbors == 3 {
            new_cubes.insert(active_cube.clone());
        }
    }

    for inactive_cube in to_check {
        let active_neighbors = inactive_cube
            .neighbors()
            .iter()
            .filter(|cube| cubes.contains(cube))
            .count();
        if active_neighbors == 3 {
            new_cubes.insert(inactive_cube);
        }
    }

    new_cubes
}

fn main() {
    let input = "data/input1.txt";
    let mut cubes = load_active_cubes(input);
    // paint(&cubes);
    for ciclo in 0..6 {
        println!("Ciclo {}", ciclo);
        cubes = evolve(cubes);
        // paint(&cubes);
        println!();
    }
    dbg!(cubes.len());
}
