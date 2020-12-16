use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::RangeInclusive;

type Num = u16;
type Rule = (String, RangeInclusive<Num>, RangeInclusive<Num>);
type Ticket = Vec<Num>;

fn main() {
    let (rules, my_ticket, tickets) = load("data/input1.txt");

    println!("Rules");
    for r in &rules {
        println!("{:>20}: {:>8}, {:?}", r.0, format!("{:?}", r.1), r.2);
    }
    println!();

    let ticket_ops = part_1(&rules, tickets);

    let fits = part_2(&rules, ticket_ops);
    let mut ans: usize = 1;
    for (name, pos) in fits {
        if name.starts_with("departure") {
            ans *= my_ticket[pos] as usize;
        }
    }
    dbg!(ans);
}

fn part_2(rules: &[Rule], ticket_ops: Vec<Vec<Num>>) -> HashMap<String, usize> {
    let initial_fits: HashSet<String> = rules.iter().map(|(name, _, _)| name.clone()).collect();
    let mut known_fits: HashMap<usize, String> = HashMap::with_capacity(rules.len());
    let mut fits: HashMap<usize, HashSet<String>> = (0..rules.len())
        .map(|i| (i, initial_fits.clone()))
        .collect();
    for (idx, column) in ticket_ops.iter().enumerate() {
        for col_opt in column {
            for (rule_name, first_range, second_range) in rules {
                if !first_range.contains(col_opt) && !second_range.contains(col_opt) {
                    fits.get_mut(&idx).unwrap().remove(rule_name);
                }
            }
        }
    }
    dbg!(&fits);

    while fits.iter().any(|(_, set)| !set.is_empty()) {
        let (idx, to_remove) = fits
            .iter()
            .find(|(_, set)| set.len() == 1)
            .map(|(col, set)| (*col, set.iter().next().unwrap().clone()))
            .unwrap();
        known_fits.insert(idx, to_remove.clone());
        for set in fits.values_mut() {
            set.remove(&to_remove);
        }
    }

    let known_fits = known_fits
        .into_iter()
        .map(|(idx, name)| (name, idx))
        .collect();
    dbg!(known_fits)
}

fn part_1(rules: &[Rule], tickets: Vec<Ticket>) -> Vec<Vec<Num>> /* options for each ticket column */
{
    let mut error_rate = 0;
    let mut tickt_ops = Vec::with_capacity(rules.len());
    for _ in 0..rules.len() {
        tickt_ops.push(vec![]);
    }

    for t in tickets.into_iter() {
        let mut ticket_is_valid = true;
        for n in &t {
            let mut is_valid = false;
            for (_rule_name, first_range, second_range) in rules {
                if first_range.contains(&n) || second_range.contains(&n) {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                ticket_is_valid = false;
                error_rate += n;
            }
        }
        if ticket_is_valid {
            for (idx, op) in t.into_iter().enumerate() {
                tickt_ops[idx].push(op);
            }
        }
    }

    dbg!(error_rate);
    tickt_ops
}

fn load(filename: &'static str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let contents = read_to_string(filename).unwrap();
    let mut components = contents.split("\n\n");
    let rules = components
        .next()
        .unwrap()
        .replace(" or", ":")
        .replace("-", ": ")
        .lines()
        .map(|l| {
            let mut l = l.split(": ");
            let rule_name = l.next().unwrap();
            let first_range =
                l.next().unwrap().parse().unwrap()..=l.next().unwrap().parse().unwrap();
            let second_range =
                l.next().unwrap().parse().unwrap()..=l.next().unwrap().parse().unwrap();
            (rule_name.to_string(), first_range, second_range)
        })
        .collect();
    let my_ticket = components
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let tickets = components
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, my_ticket, tickets)
}
