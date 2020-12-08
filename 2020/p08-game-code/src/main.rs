use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum Op {
    Acc(isize),
    Jmp(isize),
    NOp(isize),
}

impl Op {
    fn inverse(&self) -> Self {
        match self {
            Op::Acc(_) => unreachable!(),
            Op::NOp(num) => Op::Jmp(*num),
            Op::Jmp(num) => Op::NOp(*num),
        }
    }
}

type Accum = isize;
type Pointer = isize;

#[derive(Debug)]
struct ItLoops<'a>(Accum, Pointer, &'a Op);

fn load_instructions(file_name: &'static str) -> Vec<Op> {
    read_to_string(file_name)
        .expect("bad input")
        .lines()
        .map(|l| {
            let num = l[4..].parse().unwrap();
            match &l[..3] {
                "acc" => Op::Acc(num),
                "jmp" => Op::Jmp(num),
                "nop" => Op::NOp(num),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn execute(pointer: &mut Pointer, acc: &mut Accum, op: &Op) {
    match op {
        Op::NOp(_) => *pointer += 1,
        Op::Jmp(num) => *pointer += num,
        Op::Acc(num) => {
            *pointer += 1;
            *acc += num
        }
    }
}

fn loops<'a, 'b>(
    starting_pointer: Pointer,
    already_visited: &'a HashSet<Pointer>,
    starting_acc: Accum,
    instructions: &'b [Op],
) -> Result<Accum, ItLoops<'b>> {
    let mut visited_pointers: HashSet<isize> = HashSet::default();
    let mut acc = starting_acc;
    let mut pointer = starting_pointer;
    let ending = instructions.len();
    loop {
        if ending as isize == pointer {
            return Ok(acc);
        } else if visited_pointers.contains(&pointer) || already_visited.contains(&pointer) {
            let op = &instructions[pointer as usize];
            return Err(ItLoops(acc, pointer, op));
        } else {
            let op = &instructions[pointer as usize];
            visited_pointers.insert(pointer);
            execute(&mut pointer, &mut acc, op);
        }
    }
}

fn main() {
    let mut instructions = load_instructions("data/input1.txt");
    let mut visited_pointers: HashSet<Pointer> = HashSet::default();
    let mut acc = 0;
    let mut pointer: Pointer = 0;

    loop {
        // try changing one at a time and executing the whole program. If it loops, advance one
        // operation
        let original_op = instructions[pointer as usize].clone();
        match original_op {
            Op::Acc(_) => {
                // this one does not change, simply execute it
                visited_pointers.insert(pointer);
                execute(&mut pointer, &mut acc, &original_op);
            }
            _ => {
                // change it
                instructions[pointer as usize] = original_op.inverse();
                match loops(pointer, &visited_pointers, acc, &instructions) {
                    Ok(acc) => {
                        return println!("Accumulator changing {:?} ends in {}", original_op, acc);
                    }
                    Err(_) => {
                        // revert, visit and execute
                        instructions[pointer as usize] = original_op.clone();
                        visited_pointers.insert(pointer);
                        execute(&mut pointer, &mut acc, &original_op);
                    }
                }
            }
        }
    }
}
