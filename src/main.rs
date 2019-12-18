use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

fn parse_part(part: &str) -> (&str, usize) {
    let (amount, chemical) = part.split_at(part.find(' ').expect("whitespace not found"));
    (
        chemical.trim(),
        amount
            .trim()
            .parse::<usize>()
            .expect("bad amount in chemical"),
    )
}

fn parse_line(line: &str) -> (&str, (usize, Vec<(&str, usize)>)) {
    let mut parts = line.split(" => ");
    let components = parts.next().unwrap().split(", ").map(parse_part).collect();
    let (r_chemical, r_amount) = parse_part(parts.next().unwrap());
    (r_chemical, (r_amount, components))
    // unimplemented!();
}

fn main() {
    assert_eq!(find_ore("data/test0.txt", false), 165);
    assert_eq!(find_ore("data/test1.txt", false), 31);
    assert_eq!(find_ore("data/test2.txt", false), 13312);
    assert_eq!(find_ore("data/test3.txt", false), 180697);
    assert_eq!(find_ore("data/test4.txt", true), 2210736);
}

fn find_ore(path: &str, debug: bool) -> usize {
    let raw = read_to_string(path).expect("error reading file");
    let reaction_info: HashMap<_, _> = raw.lines().map(parse_line).collect();
    let mut shelf = HashMap::new();
    let mut stack = vec![(1_usize, "FUEL")];
    let mut ore = 0;
    let mut iters = 0;
    while let Some((mut needed_amount, obj)) = stack.pop() {
        if debug {
            println!("[{}] next one is {} {}", iters, needed_amount, obj);
        }
        // how much do I have of the objective chemical
        let have = *shelf.entry(obj).or_insert(0);
        if debug {
            println!("\tof {} I have {}", obj, have);
        }
        // how much says the reaction I can produce
        let (produced_amount, needed_chemicals) = reaction_info
            .get(obj)
            .expect("component not found in reaction_info");
        // use any amount I already have
        if have >= needed_amount {
            needed_amount = 0;
            shelf.insert(obj, have - needed_amount);
        } else if have < needed_amount {
            needed_amount -= have;
            shelf.insert(obj, 0);
        }
        if debug {
            println!(
                "\tI updated the needed_amount({}) and what I have({})",
                needed_amount, have
            );
            println!("\tfound that I can produce {} of {}", produced_amount, obj);
        }
        let factor = needed_amount.div_euclid(*produced_amount)
            + if needed_amount.rem_euclid(*produced_amount) > 0 {
                1
            } else {
                0
            };
        if debug {
            println!("\tconcluded that the factor is {}", factor);
        }
        *shelf.entry(obj).or_insert(0) += (factor * produced_amount - needed_amount);
        if debug {
            println!(
                "\t now I have {} - {} = {:?}",
                produced_amount * factor,
                needed_amount,
                shelf.get(obj)
            );
        }

        for (next_req_chem, next_req_am) in needed_chemicals {
            let next_need = factor * next_req_am;
            if next_req_chem == &"ORE" {
                if debug {
                    println!("\tfound ORE, had {} and will add {}", ore, next_need);
                }
                ore += next_req_am * factor;
                // *shelf.entry(obj).or_insert(0) += (factor * produced_amount - needed_amount);
                // if debug {
                //     println!(
                //         "\t now I have {} - {} = {:?}",
                //         produced_amount * factor,
                //         needed_amount,
                //         shelf.get(obj)
                //     );
                // }
                continue;
            }
            if debug {
                println!(
                    "\tI'm adding {} of {} to the stack",
                    next_need, next_req_chem
                );
            }
            stack.push((next_need, next_req_chem));
            // *shelf.entry(obj).or_insert(0) += (factor * produced_amount - needed_amount);
            // if debug {
            //     println!(
            //         "\t now I have {} - {} = {:?}",
            //         produced_amount * factor,
            //         needed_amount,
            //         shelf.get(obj)
            //     );
            // }
        }
        if debug {
            let mut inp = String::new();
            io::stdin().read_line(&mut inp).unwrap();
            let c = inp.chars().next().unwrap();
            while c != 'c' {
                match c {
                    _ => break,
                }
            }
        }
        iters += 1;
    }
    println!("{} required ore with {} iters", ore, iters);
    ore
}
