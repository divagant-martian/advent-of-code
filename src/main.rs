use std::env;
use std::fs::read_to_string;

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
    match num.rem_euclid(100) {
        1 => {
            let aux = num.div_euclid(100);
            Opcode::Add(aux.rem_euclid(10) == 1, aux.div_euclid(10) == 1)
        }
        2 => {
            let aux = num.div_euclid(100);
            Opcode::Multiply(aux.rem_euclid(10) == 1, aux.div_euclid(10) == 1)
        }
        3 => Opcode::Input,
        4 => Opcode::Output(num.div_euclid(100).rem_euclid(10) == 1),
        5 => {
            let aux = num.div_euclid(100);
            Opcode::JumpIfTrue(aux.rem_euclid(10) == 1, aux.div_euclid(10) == 1)
        }
        6 => {
            let aux = num.div_euclid(100);
            Opcode::JumpIfFalse(aux.rem_euclid(10) == 1, aux.div_euclid(10) == 1)
        }
        7 => {
            let aux = num.div_euclid(100);
            Opcode::LessThan(aux.rem_euclid(10) == 1, aux.div_euclid(10) == 1)
        }
        8 => {
            let aux = num.div_euclid(100);
            Opcode::Equals(aux.rem_euclid(10) == 1, aux.div_euclid(10) == 1)
        }

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
        _ => pointer,
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

fn input(pointer: usize, program: &mut [i32]) -> usize {
    let input = 1;
    program[program[pointer + 1] as usize] = input;
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

fn halt(pointer: usize, program: &mut [i32]) -> usize {
    println!("Program: {}", program[0]);
    pointer
}

fn run(program: &mut [i32]) {
    let mut pointer = 0;
    let mut op: Opcode;
    let mut new_pointer = 0;

    while {
        op = from_num(program[pointer]);
        // println!("P:{:03} N:{:?} {:?}", pointer, program[pointer], op);
        new_pointer = execute(op, pointer, program);
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
