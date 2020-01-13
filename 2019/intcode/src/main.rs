use intcode::program::{Int, Program};
use intcode::{get_data_from_path, solution_7a, solution_7b};
use std::env;

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
    let data = get_data_from_path(&path);
    match args.next() {
        Some(x) => match x.as_str() {
            "7a" => println!("{:?}", solution_7a::run_solution(&data, false)),
            "7a_dbg" => println!("{:?}", solution_7a::run_solution(&data, true)),
            "7b" => println!("{:?}", solution_7b::run_solution(data, false)),
            "dbg" => simple_run(&data, true),
            _ => panic!("what?"),
        },
        None => simple_run(&data, false),
    }
}
