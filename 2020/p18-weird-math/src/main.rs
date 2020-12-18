use std::fs::read_to_string;

#[derive(Debug)]
enum Parts {
    LeftP,
    Num(isize),
    Add(isize),
    Mult(isize),
}

use Parts::*;

#[derive(PartialEq, Eq)]
enum Op {
    Add,
    Mult,
    LeftP,
}

impl Op {
    fn from_char(c: char) -> Option<Op> {
        match c {
            '+' => Some(Op::Add),
            '*' => Some(Op::Mult),
            '(' => Some(Op::LeftP),
            _ => None,
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            Op::Add => 2,
            Op::Mult => 1,
            Op::LeftP => 0,
        }
    }
}

fn eval_line_part_2(line: &str) -> isize {
    let mut output_stack = Vec::new();
    let mut op_stack: Vec<Op> = Vec::new();
    for c in line.chars().filter(|c| *c != ' ') {
        match c {
            '+' | '*' => {
                let op = Op::from_char(c).unwrap();
                while let Some(prev) = op_stack.pop() {
                    if prev.precedence() >= op.precedence() {
                        let x = output_stack.pop().unwrap();
                        let y = output_stack.pop().unwrap();
                        match prev {
                            Op::Mult => output_stack.push(x * y),
                            Op::Add => output_stack.push(x + y),
                            Op::LeftP => unreachable!("dafuck"),
                        }
                    } else {
                        op_stack.push(prev);
                        break;
                    }
                }
                op_stack.push(op);
            }
            '(' => op_stack.push(Op::LeftP),
            ')' => {
                while let Some(prev) = op_stack.pop() {
                    match prev {
                        Op::LeftP => {
                            break;
                        }
                        Op::Add => {
                            let x = output_stack.pop().unwrap();
                            let y = output_stack.pop().unwrap();
                            output_stack.push(x + y)
                        }
                        Op::Mult => {
                            let x = output_stack.pop().unwrap();
                            let y = output_stack.pop().unwrap();
                            output_stack.push(x * y)
                        }
                    }
                }
            }
            n => {
                let n: isize = n.to_digit(10).unwrap() as isize;
                output_stack.push(n);
            }
        }
    }

    while let Some(prev) = op_stack.pop() {
        match prev {
            Op::LeftP => panic!("left p after loop"),
            Op::Add => {
                let x = output_stack.pop().unwrap();
                let y = output_stack.pop().unwrap();
                output_stack.push(x + y)
            }
            Op::Mult => {
                let x = output_stack.pop().unwrap();
                let y = output_stack.pop().unwrap();
                output_stack.push(x * y)
            }
        }
    }

    let ans = output_stack.pop().unwrap();
    assert!(output_stack.is_empty());
    ans
}

fn eval_line_part_1(line: &str) -> isize {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(Parts::LeftP),
            ')' => match stack.pop().unwrap() {
                Num(n) => {
                    let last = stack.pop();
                    assert!(matches!(last, Some(LeftP)), ") got before a {:?}", last);
                    match stack.pop() {
                        None => stack.push(Num(n)),
                        Some(Add(x)) => stack.push(Num(n + x)),
                        Some(Mult(x)) => stack.push(Num(n * x)),
                        Some(Num(_)) => unreachable!("num num in )"),
                        Some(LeftP) => {
                            stack.push(LeftP);
                            stack.push(Num(n))
                        }
                    }
                }
                thing => unreachable!("{:?} and then )", thing),
            },
            '+' => match stack.pop().unwrap() {
                Num(x) => stack.push(Add(x)),
                _ => unreachable!("++"),
            },
            '*' => match stack.pop().unwrap() {
                Num(x) => stack.push(Mult(x)),
                _ => unreachable!("++"),
            },
            ' ' => (),
            _ => {
                let num = c.to_digit(10).unwrap() as isize;
                match stack.pop() {
                    None => stack.push(Num(num)),
                    Some(Add(x)) => stack.push(Num(num + x)),
                    Some(Mult(x)) => stack.push(Num(num * x)),
                    Some(LeftP) => {
                        stack.push(LeftP);
                        stack.push(Num(num));
                    }
                    Some(Num(_)) => unreachable!("num num"),
                }
            }
        }
    }
    if let Some(Num(n)) = stack.pop() {
        n
    } else {
        panic!("evaluation failed")
    }
}

fn main() {
    let input = read_to_string("data/input1.txt").unwrap();
    println!(
        "Part 1 {}",
        input.lines().map(|l| eval_line_part_1(l)).sum::<isize>()
    );

    println!(
        "Part 2 {}",
        input.lines().map(|l| eval_line_part_2(l)).sum::<isize>()
    );
}
