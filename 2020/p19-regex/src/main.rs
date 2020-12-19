use crate::rules::{parse::parse, Rule, RuleIdx};
use std::collections::HashMap;
use std::fs::read_to_string;

mod rules;

fn main() {
    let filename = std::env::args().nth(1).expect("no path given");
    println!("Running with {}", filename);
    let input = read_to_string(filename).unwrap();
    let mut parts = input.split("\n\n");

    let mut rules: HashMap<RuleIdx, Rule> = parts.next().unwrap().lines().map(parse).collect();
    let rule_zero = rules.remove(&0).unwrap();
    println!("{}", rule_zero);

    println!(
        "NUMBER OF MATCHES: {}",
        parts
            .next()
            .unwrap()
            .lines()
            .filter(|l| {
                let matches = rule_zero.matches_to_end(l, &rules);
                println!("[{}] = {}", l, matches);
                matches
            })
            .count()
    );
}
