use colored::Colorize;
use intcode::get_data_from_path;
use intcode::program::{Int, Program};
use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let data = get_data_from_path("data/input.txt");
    let (in_s, in_r) = channel();
    let (out_s, out_r) = channel();
    thread::spawn(move || loop {
        let mut prompt = String::new();
        while let Ok(x) = out_r.recv_timeout(Duration::from_millis(10)) {
            prompt.push(x as u8 as char);
        }
        if !prompt.is_empty() {
            let mut is_inventory = false;
            for part in prompt.lines() {
                if part.starts_with("=") {
                    println!("{}", part.bright_cyan().bold());
                } else if part.starts_with("-") {
                    if is_inventory {
                        println!("{}", part.bold().bright_magenta());
                    } else {
                        println!("{}", part.bold());
                    }
                } else if part.starts_with("Items in your inventory:") {
                    is_inventory = true;
                    println!("{}", part.bright_magenta().bold());
                } else if !part.is_empty() {
                    println!("{}", part);
                }
            }
        }
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();
        if !inp.is_empty() {
            for n in inp.chars().map(|c| c as u8 as Int) {
                in_s.send(n);
            }
        }
    });
    let mut prog = Program::new(&data, in_r, out_s);
    prog.run();
    thread::sleep_ms(200);
}
