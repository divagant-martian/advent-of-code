use intcode::get_data_from_path;
use intcode::program::{Int, Program};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use Direction::*;

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn left(&mut self) {
        *self = match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn right(&mut self) {
        *self = match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

pub fn doshit(start: Int) -> HashMap<(Int, Int), Int> {
    let (input_sender, input_receiver) = channel();
    let (output_sender, output_receiver) = channel();
    let mut painted_cells = HashMap::new();
    let data = get_data_from_path("data/final.txt");
    let (mut x, mut y) = (0, 0);
    let mut direction = Direction::Up;
    let mut i = 0;

    let prog_thread = thread::spawn(move || {
        let mut prog = Program::new(&data, input_receiver, output_sender);
        prog.run();
    });
    input_sender.send(start).unwrap();

    while let Ok(out) = output_receiver.recv() {
        if i % 2 == 0 {
            // first output: paint
            painted_cells.insert((x, y), out);
            i += 1;
            continue;
        }
        if out == 0 {
            direction.left(); // second output and it is a 0 (turn left)
        } else {
            direction.right(); // second output and it is a 1 (turn right)
        }
        match direction {
            Up => y += 1,
            Left => x -= 1,
            Down => y -= 1,
            Right => x += 1,
        }
        if input_sender
            .send(painted_cells.get(&(x, y)).cloned().unwrap_or_default())
            .is_err()
        {
            println!("program halted");
        }
        i += 1;
    }

    prog_thread.join().unwrap();
    painted_cells
}
