use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
enum InsA {
    Update { pos: u64, val: u64 },
    Mask { zero_mask: u64, one_mask: u64 },
}

fn get_instruction_set_a(filename: &'static str) -> Vec<InsA> /* instruction vector */ {
    let lines = read_to_string(filename)
        .unwrap()
        .replace("mem[", "")
        .replace("]", "");
    let inst = lines
        .lines()
        .map(|l| {
            if let Some(mask) = l.strip_prefix("mask = ") {
                let zero_mask = mask.clone().replace("X", "1");
                let one_mask = mask.replace("X", "0");
                let one_mask = u64::from_str_radix(&one_mask, 2).unwrap();
                let zero_mask = u64::from_str_radix(&zero_mask, 2).unwrap();
                InsA::Mask {
                    one_mask,
                    zero_mask,
                }
            } else {
                let mut split = l.split(" = ");
                let pos = split.next().unwrap().parse().unwrap();
                let val = split.next().unwrap().parse().unwrap();
                InsA::Update { pos, val }
            }
        })
        .collect();
    inst
}

fn part_a(filename: &'static str) {
    let inst = get_instruction_set_a(filename);
    let mut mem = HashMap::new();
    let (mut one_mask, mut zero_mask) = match inst.first() {
        Some(InsA::Mask {
            one_mask,
            zero_mask,
        }) => (*one_mask, *zero_mask),
        _ => unreachable!(),
    };
    for instruction in inst {
        match instruction {
            InsA::Update { pos, val } => {
                mem.insert(pos, val & zero_mask | one_mask);
            }
            InsA::Mask {
                zero_mask: new_zero,
                one_mask: new_one,
            } => {
                one_mask = new_one;
                zero_mask = new_zero;
            }
        }
    }
    let sum: u64 = mem.into_iter().map(|(_i, x)| x).sum();
    dbg!(sum);
}

enum InsB {
    Mask { one_mask: u64, x_mask: Vec<u8> },
    Update { pos: u64, val: u64 },
}

const MAX_VAL: u64 = (1 << 36) - 1;

fn get_instruction_set_b(filename: &'static str) -> Vec<InsB> /* instruction vector */ {
    let lines = read_to_string(filename)
        .unwrap()
        .replace("mem[", "")
        .replace("]", "");
    let inst = lines
        .lines()
        .map(|l| {
            if let Some(mask) = l.strip_prefix("mask = ") {
                let one_mask = mask.replace("X", "0");
                let one_mask = u64::from_str_radix(&one_mask, 2).unwrap();
                let x_mask = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter_map(|(idx, c)| if c == 'X' { Some(idx as u8) } else { None })
                    .collect();
                InsB::Mask { one_mask, x_mask }
            } else {
                let mut split = l.split(" = ");
                let pos = split.next().unwrap().parse().unwrap();
                let val = split.next().unwrap().parse().unwrap();
                InsB::Update { pos, val }
            }
        })
        .collect();
    inst
}

fn generate_masks(x_positions: &[u8], output: &mut Vec<u64>, cinta: &mut u64) {
    output.clear();
    *cinta = 0;
    for pos in x_positions {
        *cinta += 2_u64.pow((*pos).into());
    }
    *cinta = !*cinta & MAX_VAL;
    for pos in x_positions {
        let one_n = 2_u64.pow((*pos).into());
        if output.is_empty() {
            output.push(0);
            output.push(one_n);
        } else {
            for one_m in output.clone() {
                let new_one = one_m | one_n;
                output.push(new_one)
            }
        }
    }
}

fn part_b(filename: &'static str) {
    let inst = get_instruction_set_b(filename);
    let mut masks = Vec::with_capacity(2_usize.pow(9));
    let mut one_mask = 0;
    let mut cinta = 0;
    let mut mem = HashMap::new();
    for instruction in inst {
        match instruction {
            InsB::Update { pos, val } => {
                for one_m in &masks {
                    let zero_m = cinta | (one_m & MAX_VAL);
                    let masked = pos & zero_m | one_m | one_mask;
                    mem.insert(masked, val);
                }
            }
            InsB::Mask {
                one_mask: new_one,
                x_mask,
            } => {
                one_mask = new_one;
                generate_masks(&x_mask, &mut masks, &mut cinta);
            }
        }
    }
    let sum: u64 = mem.into_iter().map(|(_i, x)| x).sum();
    dbg!(sum);
}

fn main() {
    let data = "data/chris.txt";
    part_a(data);
    part_b(data);
}
