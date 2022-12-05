use std::collections::VecDeque;

use crate::{Move, Stack};

pub fn parse_data(data: &str) -> Result<(Vec<Stack>, Vec<Move>), &'static str> {
    let (stack_data, instructions) = data
        .split_once("\n\n")
        .ok_or("File whould have two parts")?;
    let mut stacks = Vec::new();
    for l in stack_data.lines() {
        for (i, c) in l
            .char_indices()
            .filter(|(i, c)| i % 4 == 1 && !c.is_numeric())
        {
            let idx = i / 4;
            if stacks.len() <= idx {
                if c != ' ' {
                    stacks.push(VecDeque::from([c]));
                } else {
                    stacks.push(VecDeque::new());
                }
            } else if c != ' ' {
                stacks.get_mut(idx).expect("stack exists").push_front(c);
            }
        }
    }

    let instructions = instructions
        .lines()
        .map(|l| {
            let l = l
                .replace("move ", "")
                .replace(" from ", " ")
                .replace(" to ", " ");
            let mut parts = l.split(' ').map(|part| {
                part.parse::<usize>()
                    .map_err(|_| "bad number in instruction")
            });
            let count = parts.next().ok_or("malformed instruction")??;
            let from_stack = parts.next().ok_or("malformed instruction")?? - 1;
            let to_stack = parts.next().ok_or("malformed instruction")?? - 1;
            Ok(Move {
                from_stack,
                to_stack,
                count,
            })
        })
        .collect::<Result<Vec<Move>, &'static str>>()?;
    Ok((stacks, instructions))
}
