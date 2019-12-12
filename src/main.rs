use std::env;
use std::fs::read_to_string;
use std::io;

#[derive(Debug)]
enum Opcode {
    Add(bool, bool),
    Multiply(bool, bool),
    JumpIfTrue(bool, bool),
    JumpIfFalse(bool, bool),
    LessThan(bool, bool),
    Equals(bool, bool),
    Input,
    Output(bool),
    Halt,
}

fn from_num(num: i32) -> Opcode {
    let aux = num.div_euclid(100);
    let m0 = aux.rem_euclid(10) == 1;
    let m1 = aux.div_euclid(10) == 1;
    match num.rem_euclid(100) {
        1 => Opcode::Add(m0, m1),
        2 => Opcode::Multiply(m0, m1),
        3 => Opcode::Input,
        4 => Opcode::Output(m0),
        5 => Opcode::JumpIfTrue(m0, m1),
        6 => Opcode::JumpIfFalse(m0, m1),
        7 => Opcode::LessThan(m0, m1),
        8 => Opcode::Equals(m0, m1),
        99 => Opcode::Halt,
        _ => panic!("bad code from num {}", num),
    }
}

fn get_param(position: usize, inmediate_mode: bool, program: &[i32]) -> i32 {
    if inmediate_mode {
        return program[position];
    }
    program[program[position] as usize]
}

/// Dispatchs the corresponding operation and returns the position increment
fn execute(code: Opcode, pointer: usize, program: &mut [i32]) -> usize {
    match code {
        Opcode::Add(m0, m1) => add(m0, m1, pointer, program),
        Opcode::Multiply(m0, m1) => multiply(m0, m1, pointer, program),
        Opcode::Input => input(pointer, program),
        Opcode::Output(m0) => output(m0, pointer, program),
        Opcode::Halt => halt(pointer, program),
        Opcode::Equals(m0, m1) => equals(m0, m1, pointer, program),
        Opcode::JumpIfTrue(m0, m1) => jump_if_true(m0, m1, pointer, program),
        Opcode::JumpIfFalse(m0, m1) => jump_if_false(m0, m1, pointer, program),
        Opcode::LessThan(m0, m1) => less_than(m0, m1, pointer, program),
    }
}

fn add(m0: bool, m1: bool, pointer: usize, program: &mut [i32]) -> usize {
    program[program[pointer + 3] as usize] =
        get_param(pointer + 1, m0, program) + get_param(pointer + 2, m1, program);
    pointer + 4
}

fn multiply(m0: bool, m1: bool, pointer: usize, program: &mut [i32]) -> usize {
    program[program[pointer + 3] as usize] =
        get_param(pointer + 1, m0, program) * get_param(pointer + 2, m1, program);
    pointer + 4
}

fn jump_if_true(m0: bool, m1: bool, pointer: usize, program: &mut [i32]) -> usize {
    if get_param(pointer + 1, m0, program) != 0 {
        return get_param(pointer + 2, m1, program) as usize;
    }
    pointer + 3
}

fn jump_if_false(m0: bool, m1: bool, pointer: usize, program: &mut [i32]) -> usize {
    if get_param(pointer + 1, m0, program) == 0 {
        return get_param(pointer + 2, m1, program) as usize;
    }
    pointer + 3
}

fn less_than(m0: bool, m1: bool, pointer: usize, program: &mut [i32]) -> usize {
    if get_param(pointer + 1, m0, program) < get_param(pointer + 2, m1, program) {
        program[program[pointer + 3] as usize] = 1;
    } else {
        program[program[pointer + 3] as usize] = 0;
    }
    pointer + 4
}

fn equals(m0: bool, m1: bool, pointer: usize, program: &mut [i32]) -> usize {
    if get_param(pointer + 1, m0, program) == get_param(pointer + 2, m1, program) {
        program[program[pointer + 3] as usize] = 1;
    } else {
        program[program[pointer + 3] as usize] = 0;
    }
    pointer + 4
}

fn input(pointer: usize, program: &mut [i32]) -> usize {
    println!("Input please, human: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    program[program[pointer + 1] as usize] = n;
    pointer + 2
}

fn output(m0: bool, pointer: usize, program: &mut [i32]) -> usize {
    if m0 {
        print!("[{}] ", program[pointer + 1]);
        return pointer + 2;
    }
    print!("[{}] ", program[program[pointer + 1] as usize]);
    // println!("program til now: {:?}\n\n", &program[0..pointer]);
    pointer + 2
}

fn halt(pointer: usize, _program: &mut [i32]) -> usize {
    pointer
}

fn run(program: &mut [i32]) {
    let mut pointer = 0;
    let mut op: Opcode;
    let mut new_pointer;

    while {
        op = from_num(program[pointer]);
        // println!("P:{:03} N:{:?} {:?}", pointer, program[pointer], op);
        new_pointer = execute(op, pointer, program);
        // println!("program: {:?}\n\n", &program);

        new_pointer != pointer
    } {
        pointer = new_pointer;
    }
}

fn main() {
    let path: String = env::args().nth(1).expect("no data path provided");
    let mut program = read_to_string(&path)
        .expect("bad input")
        .trim()
        .split(",")
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect::<Vec<i32>>();

    run(&mut program);
}
