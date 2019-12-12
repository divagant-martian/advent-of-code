mod program;
use program::Program;
use std::env;
use std::fs::read_to_string;

fn main() {
    let path: String = env::args().nth(1).expect("no data path provided");
    let data = read_to_string(&path)
        .expect("bad input")
        .trim()
        .split(',')
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<i32>>();

    let mut input = vec![];
    input.push(1);

    let mut output = vec![];

    let mut program = Program::new(&data, &mut input, &mut output);
    program.run();
    println!("OUTPUT: {:?}", output);
}
