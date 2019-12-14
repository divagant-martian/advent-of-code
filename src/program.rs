use crate::opcode::*;
use colored::*;
use std::fmt::Debug;
use std::io;
use std::io::Write;

pub struct Program<S: ProgSender, R: ProgReceiver> {
    mem: Vec<i32>,
    pointer: usize,
    input: R,
    output: S,
    feedback: bool,
}

pub trait ProgSender: Debug {
    fn put(&mut self, num: i32);
}

pub trait ProgReceiver: Debug {
    fn get(&mut self) -> Option<i32>;
}

impl<S: ProgSender, R: ProgReceiver> Program<S, R> {
    pub fn new_with_feedback(data: &Vec<i32>, input: R, output: S) -> Self {
        Program {
            mem: data.clone(),
            pointer: 0,
            input,
            output,
            feedback: true,
        }
    }

    pub fn new(data: &Vec<i32>, input: R, output: S) -> Self {
        Program {
            mem: data.clone(),
            pointer: 0,
            input,
            output,
            feedback: false,
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
        let n: i32 = match self.input.get() {
            Some(x) => x,
            None => {
                let mut inp = String::new();
                print!("Input please, human: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut inp).unwrap();
                let nn = inp.trim().parse();
                if nn.is_err() {
                    self.input();
                    return;
                }
                nn.unwrap()
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
        println!("{}", out);
        self.output.put(out);
        self.pointer += 2;
    }

    fn halt(&mut self) {}

    fn debug(&self, last_code: Opcode) {
        let dbg = "[Debug] ".green();
        let mut c: char;
        let mut inp: String;
        while {
            print!(
                "{}lastop({:^24}) pointer({:^3}) $ ",
                dbg,
                format!("{:?}", last_code),
                self.pointer
            );
            io::stdout().flush().unwrap();
            inp = String::new();
            io::stdin().read_line(&mut inp).unwrap();
            c = inp.chars().next().unwrap();
            c != 'c'
        } {
            match c {
                'm' => {
                    let mut parts = inp.splitn(3, ' ');
                    // println!("{:?}", parts.next());
                    // println!("{:?}", parts.next());
                    // println!("{:?}", parts.next());
                    parts.next();
                    if let Some(ini) = parts.next() {
                        if let Some(end) = parts.next() {
                            let x: usize = ini.parse().unwrap_or(0);
                            let y: usize = end.trim().parse().unwrap_or(self.mem.len() - 1);
                            println!("{}mem {}..={} {:?}", dbg, x, y, &self.mem[x..=y]);
                        } else {
                            println!("{}expected m x..=y", dbg);
                        }
                    } else {
                        println!("{}mem {:?}", dbg, self.mem);
                    }
                }
                'p' => println!("{}pointer {:?}", dbg, self.pointer),
                'i' => println!("{}input {:?}", dbg, self.input),
                'o' => println!("{}output {:?}", dbg, self.output),
                _ => break,
            }
        }
    }

    fn get_param(&mut self, position: usize, inmediate_mode: bool) -> i32 {
        match inmediate_mode {
            true => self.mem[self.pointer + position],
            false => self.mem[self.mem[self.pointer + position] as usize],
        }
    }

    pub fn run_debug_mode(&mut self) {
        let mut op: Opcode;
        let mut old_pointer;
        println!(
            "{}",
            "[Debug] pick
                 \n [c]     continue\
                 \n [m x y] view mem in range x..=y, ignore = view all \
                 \n [p]     view pointer\
                 \n [i]     view input stack\
                 \n [o]     view output stack
                 "
            .green()
        );

        while {
            old_pointer = self.pointer;
            op = from_num(self.mem[self.pointer]);
            self.execute(op.clone());
            old_pointer != self.pointer
        } {
            self.debug(op);
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
