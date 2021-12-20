mod display;
mod ops;
mod parse;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Num {
    n: u8,
    role: Role,
    depth: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone)]
pub struct NumTree {
    inner: Vec<Num>,
}

fn push(sum_queue: &mut Vec<(u16, usize)>, new_depth: u16, new_mag: usize) {
    if let Some((prev_depth, prev_mag)) = sum_queue.last().cloned() {
        if prev_depth == new_depth {
            sum_queue.pop();
            return push(sum_queue, prev_depth - 1, prev_mag * 3 + new_mag * 2);
        }
    }
    sum_queue.push((new_depth, new_mag));
}

impl NumTree {
    pub fn checksum(&self) -> usize {
        // To check whether it's the right answer, the snailfish teacher only checks the magnitude
        // of the final sum. The magnitude of a pair is 3 times the magnitude of its left element
        // plus 2 times the magnitude of its right element. The magnitude of a regular number is
        // just that number.
        let mut sum_queue: Vec<(u16, usize)> = Vec::default();
        for Num { n, role, depth } in &self.inner {
            match role {
                Role::Left => {
                    sum_queue.push((*depth, *n as usize));
                }
                Role::Right => {
                    let (left_depth, left_magnitude) =
                        sum_queue.pop().expect("A right must have a left");
                    assert_eq!(left_depth, *depth);
                    let basket_magnitude = *n as usize * 2 + left_magnitude * 3;

                    push(&mut sum_queue, left_depth - 1, basket_magnitude);
                }
            }
        }

        sum_queue[0].1
    }
}
