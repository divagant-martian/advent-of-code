use std::collections::HashMap;
use std::fs::read_to_string;

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
}

fn main() {
    assert_eq!(find_ore(1, "data/test0.txt"), 165);
    assert_eq!(find_ore(1, "data/test1.txt"), 31);
    assert_eq!(find_ore(1, "data/test2.txt"), 13312);
    assert_eq!(find_ore(1, "data/test3.txt"), 180697);
    assert_eq!(find_ore(1, "data/test4.txt"), 2210736);
    assert_eq!(find_ore(1, "data/input.txt"), 1037742);

    assert_eq!(find_max_fuel(1_000_000_000_000, "data/test2.txt"), 82892753);
    assert_eq!(find_max_fuel(1_000_000_000_000, "data/test3.txt"), 5586022);
    assert_eq!(find_max_fuel(1_000_000_000_000, "data/test4.txt"), 460664);

    println!(
        "part 2: {}",
        find_max_fuel(1_000_000_000_000, "data/input.txt")
    );
    // assert_eq!(find_ore(1, "data/test1.txt"), 31);
    // assert_eq!(find_ore(1, "data/test2.txt"), 13312);
    // assert_eq!(find_ore(1, "data/test3.txt"), 180697);
    // assert_eq!(find_ore(1, "data/test4.txt"), 2210736);
    // assert_eq!(find_ore(1, "data/input.txt"), 1037742);
}

fn make_fuel<'a>(
    fuel_amount: usize,
    shelf: &mut HashMap<&'a str, usize>,
    reaction_info: &HashMap<&'a str, (usize, Vec<(&'a str, usize)>)>,
) -> usize {
    let mut stack = vec![(fuel_amount, "FUEL")];
    let mut ore = 0;
    while let Some((mut needed_amount, obj)) = stack.pop() {
        // how much do I have of the objective chemical
        let have = *shelf.entry(obj).or_insert(0);
        // how much says the reaction I can produce
        let (produced_amount, needed_chemicals) = reaction_info
            .get(obj)
            .expect("component not found in reaction_info");
        // use any amount I already have
        if have >= needed_amount {
            shelf.insert(obj, have - needed_amount);
            needed_amount = 0;
        } else if have < needed_amount {
            needed_amount -= have;
            shelf.insert(obj, 0);
        }
        // the factor to which multiple the producing reactiion in order to have
        // enought of objective chemical
        let factor = needed_amount.div_euclid(*produced_amount)
            + if needed_amount.rem_euclid(*produced_amount) > 0 {
                1
            } else {
                0
            };
        // update the shelf with whatever was left
        *shelf.entry(obj).or_insert(0) += factor * produced_amount - needed_amount;

        for (next_req_chem, next_req_am) in needed_chemicals {
            let next_need = factor * next_req_am;
            if next_req_chem == &"ORE" {
                ore += next_req_am * factor;
                continue;
            }
            stack.push((next_need, next_req_chem));
        }
    }
    ore
}

fn find_max_fuel(max_ore: usize, path: &str) -> usize {
    let mut remaining_ore = max_ore;

    let raw = read_to_string(path).expect("error reading file");
    let reaction_info: HashMap<_, _> = raw.lines().map(parse_line).collect();
    let mut shelf = HashMap::new();
    let ore_one_fuel = make_fuel(1, &mut shelf, &reaction_info);
    // restart the shelfs
    shelf = HashMap::new();

    // this is the amount of fuel that i can produce without using what is left
    // in the shelf
    let starting_fuel = max_ore.div_euclid(ore_one_fuel);
    let mut max_fuel = starting_fuel;
    // make the amount to update the shelfs with what remains
    remaining_ore -= make_fuel(starting_fuel, &mut shelf, &reaction_info);
    let mut aux_ore;
    while {
        aux_ore = make_fuel(1, &mut shelf, &reaction_info);
        remaining_ore > aux_ore
    } {
        max_fuel += 1;
        remaining_ore -= aux_ore;
    }
    max_fuel
}
fn find_ore(fuel_amount: usize, path: &str) -> usize {
    let raw = read_to_string(path).expect("error reading file");
    let reaction_info: HashMap<_, _> = raw.lines().map(parse_line).collect();
    let mut shelf = HashMap::new();
    let ore = make_fuel(fuel_amount, &mut shelf, &reaction_info);
    shelf.retain(|_, &mut v| v != 0);
    ore
}
