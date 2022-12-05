use crate::{Move, Stack};

pub fn execute_1(stacks: &mut Vec<Stack>, instructions: &[Move]) {
    for instruction in instructions {
        // println!("Executing {instruction:?} on {stacks:?}");
        let Move {
            from_stack,
            to_stack,
            count,
        } = instruction;
        for _ in 0..*count {
            let item = stacks[*from_stack]
                .pop_back()
                .expect("instructions are feasible");
            stacks[*to_stack].push_back(item);
        }
        // println!("result: {stacks:?}\n");
    }
}

pub fn execute_2(stacks: &mut Vec<Stack>, instructions: &[Move]) {
    for instruction in instructions {
        // println!("Executing {instruction:?} on {stacks:?}");
        let Move {
            from_stack,
            to_stack,
            count,
        } = instruction;
        let from_len = stacks[*from_stack].len();
        let to_push = stacks[*from_stack].split_off(from_len - count);
        stacks[*to_stack].extend(to_push);
        // println!("result: {stacks:?}\n");
    }
}
