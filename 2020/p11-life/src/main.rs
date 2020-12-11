use std::collections::HashMap;
use std::fs::read_to_string;

const OFFSET: Position = 1;
type Position = isize;

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

type Seats = HashMap<(Position, Position), State>;

fn load_seats(filename: &'static str) -> (Seats, usize /* width */, usize /* height */) {
    let mut seats = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, l) in read_to_string(filename)
        .expect("bad input")
        .lines()
        .enumerate()
    {
        height += 1;
        width = 0;
        for (x, c) in l.char_indices() {
            width += 1;
            let state = match c {
                '.' => State::Floor,
                '#' => State::Occupied,
                'L' => State::Empty,
                _ => unreachable!(),
            };
            seats.insert((x as isize + OFFSET, y as isize + OFFSET), state);
        }
    }
    (seats, width, height)
}

#[allow(unused)]
fn paint(seats: &Seats, width: usize, height: usize) {
    for y in 1..(height as isize) + 1 {
        let mut row = String::with_capacity(width);
        for x in 1..(width as isize) + 1 {
            let c = match seats.get(&(x, y)).unwrap() {
                State::Empty => 'L',
                State::Occupied => '#',
                State::Floor => '.',
            };
            row.push(c);
        }
        println!("{}", row);
    }
    println!()
}

fn near_occupied(seats: &Seats, pos: &(Position, Position)) -> usize {
    let &(x, y) = pos;
    let mut occupied = 0;
    for n in &[
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ] {
        occupied += seats
            .get(n)
            .map(|s| matches!(s, State::Occupied))
            .unwrap_or_default() as usize;
    }
    occupied
}

fn near_occupied2(seats: &Seats, pos: &(Position, Position)) -> usize {
    let &(x, y) = pos;
    let mut occupied = 0;
    for (x_offset, y_offset) in &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ] {
        let mut has_found = 0;
        let mut last_x = x;
        let mut last_y = y;
        while has_found == 0 {
            last_x += x_offset;
            last_y += y_offset;
            if let Some(s) = seats.get(&(last_x, last_y)) {
                match s {
                    State::Empty => break,
                    State::Occupied => has_found = 1,
                    State::Floor => (),
                }
            } else {
                break;
            }
        }
        occupied += has_found;
    }
    occupied
}

fn evolve(seats: &Seats) -> Seats {
    let mut new = HashMap::with_capacity(seats.len());
    for (pos, state) in seats {
        let near_occupied = near_occupied(seats, &pos);
        let new_state = match (state, near_occupied) {
            (State::Occupied, near) if near >= 4 => State::Empty,
            (State::Empty, 0) => State::Occupied,
            _ => state.clone(),
        };
        new.insert(*pos, new_state);
    }
    new
}

fn evolve2(seats: &Seats) -> Seats {
    let mut new = HashMap::with_capacity(seats.len());
    for (pos, state) in seats {
        let near_occupied = near_occupied2(seats, &pos);
        let new_state = match (state, near_occupied) {
            (State::Occupied, near) if near >= 5 => State::Empty,
            (State::Empty, 0) => State::Occupied,
            _ => state.clone(),
        };
        new.insert(*pos, new_state);
    }
    new
}

fn main() {
    let input = "data/chris.txt";

    /* Part 1 */
    let (mut seats, _width, _height) = load_seats(input);

    loop {
        let new = evolve(&seats);
        if new == seats {
            let mut occupied = 0;
            for x in new.values() {
                occupied += matches!(x, State::Occupied) as usize;
            }
            println!("PART 1 occupied: {}", occupied);
            break;
        }
        seats = new;
    }

    /* Part 1 */
    let (mut seats, _width, _height) = load_seats(input);

    loop {
        let new = evolve2(&seats);
        if new == seats {
            let mut occupied = 0;
            for x in new.values() {
                occupied += matches!(x, State::Occupied) as usize;
            }
            println!("PART 2 occupied: {}", occupied);
            break;
        }
        seats = new;
    }
}
