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

fn generate_masks(x_positions: &[u8], output: &mut Vec<(u64, u64)>) {
    output.clear();
    println!("positions: {:?}", x_positions);
    let mut cinta = 0;
    for pos in x_positions {
        cinta += 2_u64.pow((*pos).into());
    }
    // cinta = !cinta & MAX_VAL;
    println!("cinta: {:036b}", cinta);
    for pos in x_positions {
        let one_n = 2_u64.pow((*pos).into());
        if output.is_empty() {
            output.push((0, !cinta & MAX_VAL));
            output.push((one_n, !(cinta & !one_n) & MAX_VAL));
        } else {
            for (one_m, _zero_m) in output.clone() {
                let new_one = one_m | one_n;
                let new_zero = !(cinta & !new_one) & MAX_VAL;
                output.push((new_one, new_zero))
            }
        }
    }
    for (new_one, new_zero) in output {
        println!("mask one: {:036b}, zero: {:036b}", new_one, new_zero);
    }
    println!()
}

fn part_b(filename: &'static str) {
    let inst = get_instruction_set_b(filename);
    let mut masks = Vec::with_capacity(2_usize.pow(9));
    let mut one_mask = 0;
    let mut mem = HashMap::new();
    for instruction in inst {
        match instruction {
            InsB::Update { pos, val } => {
                for (one_m, zero_m) in &masks {
                    let masked = pos & zero_m | one_m | one_mask;
                    println!("addr:[{:03}]{:036b} val:{}", masked, masked, val);
                    mem.insert(masked, val);
                }
                println!();
            }
            InsB::Mask {
                one_mask: new_one,
                x_mask,
            } => {
                one_mask = new_one;
                generate_masks(&x_mask, &mut masks);
            }
        }
    }
    let sum: u64 = mem.into_iter().map(|(_i, x)| x).sum();
    dbg!(sum);
}

fn main() {
    let data = "data/input1.txt";
    part_a(data);
    // part_b("data/test2.txt");

    part_b(data);
}
