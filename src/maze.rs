use std::collections::HashMap;
use std::fs::read_to_string;
pub use Tile::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Floor,
    Portal([char; 2], bool),
}

pub type Maze = HashMap<(u16, u16), Tile>;

pub fn get_maze(path: &str) -> (Maze, (u16, u16)) {
    let raw = read_to_string(path).expect("problem with file");

    // missing for assignment
    let mut missing: HashMap<(u16, u16), char> = HashMap::new();
    let mut tiles = HashMap::new();
    let mut last = '%';
    let mut is_outer;
    let mut is_void;

    // vertical (y)
    for (row, line) in raw.lines().enumerate() {
        is_outer = true;
        is_void = false;
        // horizontal (x)
        for (col, c) in line.char_indices() {
            let pos = (col as u16 + 2, row as u16 + 2); // add two to overcome overflows
            if c == '.' || c == '#' {
                is_outer = false;
                if is_void {
                    is_outer = true;
                }
            }
            if !is_outer && c == ' ' {
                is_void = true;
            }
            match c {
                '.' => {
                    tiles.insert(pos, Floor);
                }
                '#' | ' ' => {} // ignore walls and empty space
                'A'..='Z' => {
                    // find left
                    if last.is_ascii_alphabetic() {
                        let portal = Portal([last, c], is_outer);
                        // Dedice if it connects left or right
                        if let Some(Floor) = tiles.get(&(pos.0 - 2, pos.1)) {
                            tiles.insert((pos.0 - 1, pos.1), portal);
                        } else {
                            tiles.insert(pos, portal);
                        }
                    } else if let Some(prev_char) = missing.remove(&(pos.0, pos.1 - 1)) {
                        let portal = Portal([prev_char, c], is_outer);
                        // this is a vertical portal. Now we need to decide if it connects up or down
                        if let Some(Floor) = tiles.get(&(pos.0, pos.1 - 2)) {
                            tiles.insert((pos.0, pos.1 - 1), portal);
                        } else {
                            tiles.insert(pos, Portal([prev_char, c], is_outer));
                        }
                    } else {
                        missing.insert(pos, c);
                    }
                }
                _ => panic!("found a strange character: {:?}", c),
            };
            last = c;
        }
    }

    let (&(x0, y0), _) = tiles
        .iter()
        .find(|(_, &v)| v == Portal(['A', 'A'], true))
        .expect("portal AA not found");
    for &opt in &[(x0 + 1, y0), (x0 - 1, y0), (x0, y0 + 1), (x0, y0 - 1)] {
        if let Some(k) = tiles.get(&opt) {
            if k == &Floor {
                return (tiles, opt);
            }
        }
    }
    unreachable!();
}

pub fn paint_maze(maze: &Maze, pos: (u16, u16)) {
    let (x_max, y_max) = maze.keys().fold((0, 0), |(x_max, y_max), &(x, y)| {
        (x_max.max(x), y_max.max(y))
    });

    let mut x_top = String::from(" ");
    let mut x_bot = String::from(" ");
    for x in 2..=x_max + 1 {
        let c = (x % 10).to_string();
        if x % 2 == 0 {
            x_top += &c;
            x_bot += " ";
        } else {
            x_top += " ";
            x_bot += &c;
        }
    }
    // println!("{}", x_top);
    for y in 3..=y_max {
        let mut row = String::new();
        let row_i = &(y % 10).to_string();
        row += row_i;
        for x in 2..=x_max {
            if (x, y) == pos {
                row.push('*');
            } else if let Some(k) = maze.get(&(x, y)) {
                match k {
                    Floor => row.push('.'),
                    Portal([_, b], true) => row.push(*b),
                    Portal([_, b], false) => row.push(b.to_ascii_lowercase()),
                }
            } else {
                row.push(' ');
            }
        }
        row += " ";
        row += row_i;
        println!("{}", row);
    }
    // println!("{}", x_bot);
    println!("");
}
