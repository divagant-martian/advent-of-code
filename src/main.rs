mod opcode;
mod program;
use itertools::Itertools;
use program::Program;
use std::env;
use std::fs::read_to_string;

fn simple_run(data: &Vec<i32>, debug: bool) {
    let mut input = vec![];
    let mut output = vec![];
    let mut program = Program::new(&data, &mut input, &mut output);
    run(&mut program, debug);
    println!("OUTPUT: {:?}", output);
}

fn run_7a(data: &Vec<i32>, debug: bool) {
    let perms = (0..5).permutations(5);
    let mut input = vec![];
    let mut output = vec![0];
    let mut max_perm = vec![-1; 5];
    let mut max = 0;
    for permutation in perms {
        for phase in &permutation {
            let last_out = output.pop().expect("last amplifier had no output");
            input.push(last_out);
            input.push(*phase);
            let mut program = Program::new(&data, &mut input, &mut output);
            run(&mut program, debug);
        }
        let perm_out = output.pop().expect("throusers without output");
        output.push(0);
        if perm_out > max {
            max = perm_out;
            max_perm = permutation;
        }
    }
    println!("max: {} produced by: {:?}", max, max_perm);
}

fn run(program: &mut Program, debug: bool) {
    match debug {
        true => program.run_debug_mode(),
        false => program.run(),
    }
}

fn run_7b(data: &Vec<i32>, debug: bool) {
    let perms = (0..5).permutations(5);
    let mut input = vec![];
    let mut output = vec![0];
    let mut max_perm = vec![-1; 5];
    let mut max = 0;
    for permutation in perms {
        for phase in &permutation {
            let last_out = output.pop().expect("last amplifier had no output");
            input.push(last_out);
            input.push(*phase);
            let mut program = Program::new(&data, &mut input, &mut output);
            run(&mut program, debug);
        }
        let perm_out = output.pop().expect("throusers without output");
        output.push(0);
        if perm_out > max {
            max = perm_out;
            max_perm = permutation;
        }
    }
    println!("max: {} produced by: {:?}", max, max_perm);
}
fn main() {
    let mut args = env::args();
    let path: String = args.nth(1).expect("no data path provided");
    let data = read_to_string(&path)
        .expect("bad input")
        .trim()
        .split(',')
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<i32>>();
    match args.next() {
        Some(x) => match x.as_str() {
            "7a" => run_7a(&data, false),
            "7a_dbg" => (),
            "7b" => run_7b(&data, true),
            "7b_dbg" => (),
            "dbg" => simple_run(&data, true),
            _ => panic!(),
        },
        None => simple_run(&data, false),
    }
}
