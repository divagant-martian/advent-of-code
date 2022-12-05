use std::collections::VecDeque;

mod execute;
mod parse;

#[derive(Debug)]
pub struct Move {
    from_stack: usize,
    to_stack: usize,
    count: usize,
}

pub type Stack = VecDeque<char>;

fn problem_1(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|stack| stack.back().expect("no stack is empty?"))
        .collect()
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Needs a file name");
    let file_contents = std::fs::read_to_string(file_name).expect("File exists");
    let (mut stacks, instructions) = parse::parse_data(&file_contents).unwrap();
    execute::execute(&mut stacks, &instructions);
    dbg!(problem_1(&stacks));
}
