use crate::opcode::*;
use std::io;

pub struct Program<'a> {
    mem: Vec<i32>,
    pointer: usize,
    input: &'a mut Vec<i32>,
    output: &'a mut Vec<i32>,
}

impl<'a> Program<'a> {
    pub fn new(data: &Vec<i32>, input: &'a mut Vec<i32>, output: &'a mut Vec<i32>) -> Self {
        Program {
            mem: data.clone(),
            pointer: 0,
            input,
            output,
        }
    }
    /// Dispatchs the corresponding operation and returns the new pointer
    fn execute(&mut self, code: Opcode) {
        match code {
            Opcode::Add(m0, m1) => self.add(m0, m1),
            Opcode::Multiply(m0, m1) => self.multiply(m0, m1),
            Opcode::Input => self.input(),
            Opcode::Output(m0) => self.output(m0),
            Opcode::Halt => self.halt(),
            Opcode::Equals(m0, m1) => self.equals(m0, m1),
            Opcode::JumpIfTrue(m0, m1) => self.jump_if_true(m0, m1),
            Opcode::JumpIfFalse(m0, m1) => self.jump_if_false(m0, m1),
            Opcode::LessThan(m0, m1) => self.less_than(m0, m1),
        }
    }

    fn add(&mut self, m0: bool, m1: bool) {
        let p = self.mem[self.pointer + 3] as usize;
        self.mem[p] = self.get_param(1, m0) + self.get_param(2, m1);
        self.pointer += 4;
    }

    fn multiply(&mut self, m0: bool, m1: bool) {
        let p = self.mem[self.pointer + 3] as usize;
        self.mem[p] = self.get_param(1, m0) * self.get_param(2, m1);
        self.pointer += 4;
    }

    fn jump_if_true(&mut self, m0: bool, m1: bool) {
        self.pointer = match self.get_param(1, m0) != 0 {
            true => self.get_param(2, m1) as usize,
            false => self.pointer + 3,
        }
    }

    fn jump_if_false(&mut self, m0: bool, m1: bool) {
        self.pointer = match self.get_param(1, m0) == 0 {
            true => self.get_param(2, m1) as usize,
            false => self.pointer + 3,
        }
    }

    fn less_than(&mut self, m0: bool, m1: bool) {
        let p = self.mem[self.pointer + 3] as usize;
        self.mem[p] = (self.get_param(1, m0) < self.get_param(2, m1)) as i32;
        self.pointer += 4;
    }

    fn equals(&mut self, m0: bool, m1: bool) {
        let p = self.mem[self.pointer + 3] as usize;
        self.mem[p] = (self.get_param(1, m0) == self.get_param(2, m1)) as i32;
        self.pointer += 4;
    }

    fn input(&mut self) {
        let n: i32 = match self.input.pop() {
            Some(x) => x,
            None => {
                let mut inp = String::new();
                println!("Input please, human: ");
                io::stdin().read_line(&mut inp).unwrap();
                inp.trim().parse().unwrap()
            }
        };
        let p = self.mem[self.pointer + 1] as usize;
        self.mem[p] = n;
        self.pointer += 2;
    }

    fn output(&mut self, m0: bool) {
        let out = match m0 {
            true => self.mem[self.pointer + 1],
            false => self.mem[self.mem[self.pointer + 1] as usize],
        };
        self.output.push(out);
        self.pointer += 2;
    }

    fn halt(&mut self) {}

    fn get_param(&mut self, position: usize, inmediate_mode: bool) -> i32 {
        match inmediate_mode {
            true => self.mem[self.pointer + position],
            false => self.mem[self.mem[self.pointer + position] as usize],
        }
    }

    pub fn run(&mut self) {
        let mut op: Opcode;
        let mut old_pointer;
        while {
            old_pointer = self.pointer;
            op = from_num(self.mem[self.pointer]);
            self.execute(op);
            old_pointer != self.pointer
        } {}
    }
}
