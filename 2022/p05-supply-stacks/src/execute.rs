use crate::{Move, Stack};

pub fn execute(stacks: &mut Vec<Stack>, instructions: &[Move]) {
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
