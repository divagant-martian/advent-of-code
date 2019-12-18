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
    assert_eq!(find_ore("data/test0.txt"), 165);
    assert_eq!(find_ore("data/test1.txt"), 31);
    assert_eq!(find_ore("data/test2.txt"), 13312);
    assert_eq!(find_ore("data/test3.txt"), 180697);
    assert_eq!(find_ore("data/test4.txt"), 2210736);
    assert_eq!(find_ore("data/input.txt"), 1037742);
}

fn find_ore(path: &str) -> usize {
    let raw = read_to_string(path).expect("error reading file");
    let reaction_info: HashMap<_, _> = raw.lines().map(parse_line).collect();
    let mut shelf = HashMap::new();
    let mut stack = vec![(1_usize, "FUEL")];
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
        let factor = needed_amount.div_euclid(*produced_amount)
            + if needed_amount.rem_euclid(*produced_amount) > 0 {
                1
            } else {
                0
            };
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
    shelf.retain(|_, &mut v| v != 0);
    println!("{} required ore", ore);
    println!("the shelf: {:?}", shelf);
    ore
}
