mod opcode;
pub mod program;
pub mod solution_7a;
pub mod solution_7b;

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

pub fn get_data_from_path(path: &str) -> Vec<Int> {
    get_data_from_str(&read_to_string(path).expect("bad input"))
}

pub fn get_data_from_str(string: &str) -> Vec<Int> {
    string
        .trim()
        .split(',')
        .map(|x| Int::from_str_radix(x, 10).unwrap())
        .collect()
}
