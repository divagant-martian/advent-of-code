use intcode::program::{Int, Program};
use intcode::{get_data_from_path, get_data_from_str};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let mut data = get_data_from_path("data/input.txt");
    data[0] = 2;
    let (output_sender, output_receiver) = channel();
    let (input_sender, input_receiver) = channel();

    let prog_thread = thread::spawn(move || {
        let mut prog = Program::new(&data, input_receiver, output_sender);
        prog.run();
        println!("game over");
    });

    let mut i = 0;
    let (mut x, mut y) = (-1, -1);
    let mut x_paddle = -1;
    let mut tiles = HashMap::new();

    while let Ok(out) = output_receiver.recv() {
        match i {
            0 => x = out,
            1 => y = out,
            _ => {
                if out == 3 {
                    x_paddle = x;
                }
                if out == 4 {
                    if input_sender.send((x - x_paddle).signum()).is_err() {
                        println!("game ended");
                    };
                }
                tiles.insert((x, y), out);
                // println!(
                //     "{} bricks remaining",
                //     tiles.values().filter(|&&v| v == 2).count()
                // );
            }
        }

        i = (i + 1) % 3;
    }
    prog_thread.join().unwrap();
    println!("{:?}", tiles.get(&(-1, 0)));
}
