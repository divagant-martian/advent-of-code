mod opcode;
mod program;
mod solution_7a;
mod solution_7b;

use program::{Int, ProgReceiver, ProgSender, Program};
use std::env;
use std::fs::read_to_string;

impl ProgSender for &mut Vec<Int> {
    fn put(&mut self, num: Int) {
        self.push(num);
    }
}

impl ProgReceiver for &mut Vec<Int> {
    fn get(&mut self) -> Option<Int> {
        self.pop()
    }
}

fn simple_run(data: &Vec<Int>, debug: bool) {
    let mut input = vec![];
    let mut output = vec![];
    let mut program = Program::new(&data, &mut input, &mut output);
    if debug {
        program.run_debug_mode();
    } else {
        program.run()
    }
    println!("OUTPUT: {:?}", output);
}

fn main() {
    let mut args = env::args();
    let path: String = args.nth(1).expect("no data path provided");
    let data = read_to_string(&path)
        .expect("bad input")
        .trim()
        .split(',')
        .map(|x| Int::from_str_radix(x, 10).unwrap())
        .collect::<Vec<Int>>();
    match args.next() {
        Some(x) => match x.as_str() {
            "7a" => solution_7a::run_solution(&data, false),
            "7a_dbg" => solution_7a::run_solution(&data, true),
            "7b" => solution_7b::run_solution(data, false),
            "7b_dbg" => (),
            "dbg" => simple_run(&data, true),
            _ => panic!(),
        },
        None => simple_run(&data, false),
    }
}
