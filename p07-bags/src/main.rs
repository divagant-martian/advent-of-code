use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;

type RuleSet = HashMap<String, Vec<(usize, String)>>;
enum State {
    ParsingSource1,
    ParsingSource2,
    ParsingQuantity,
    ParsingTarget1,
    ParsingTarget2,
}
use State::*;

enum Mode {
    MustContain,
    CanBeContained,
}

fn get_rules(file_name: &'static str, mode: Mode) -> RuleSet {
    let mut rules: RuleSet = HashMap::default();
    let mut source_color = String::new();
    let mut quantity_str = String::new();
    let mut target_color = String::new();

    let mut state = State::ParsingSource1;
    for rule in read_to_string(file_name).expect("bad input").lines() {
        for part in rule
            .replace(" bags contain ", " ")
            .replace(" bags, ", " ")
            .replace(" bags.", "")
            .replace(" bag, ", " ")
            .replace(" bag.", "")
            .replace(" no other", "")
            .split(' ')
        {
            state = match state {
                ParsingSource1 => {
                    source_color += part;
                    source_color += "_";
                    ParsingSource2
                }
                ParsingSource2 => {
                    source_color += part;
                    if matches!(mode, Mode::MustContain) {
                        rules.entry(source_color.clone()).or_insert(Vec::new());
                    }
                    ParsingQuantity
                }
                ParsingQuantity => {
                    quantity_str += part;
                    ParsingTarget1
                }
                ParsingTarget1 => {
                    target_color += part;
                    target_color += "_";
                    ParsingTarget2
                }
                ParsingTarget2 => {
                    target_color += part;
                    // parsing time!
                    match mode {
                        Mode::MustContain => rules
                            .get_mut(&source_color)
                            .unwrap()
                            .push((quantity_str.parse().unwrap(), target_color.clone())),
                        Mode::CanBeContained => rules
                            .entry(target_color.clone())
                            .or_default()
                            .push((quantity_str.parse().unwrap(), source_color.clone())),
                    }
                    quantity_str.clear();
                    target_color.clear();
                    ParsingQuantity
                }
            };
        }
        source_color.clear();
        state = ParsingSource1;
    }
    rules
}
fn print_rules(rules: &RuleSet) {
    for (source, rules) in rules {
        println!("- rules for {}", source);
        for (qty, target) in rules {
            println!("  - {} {}", qty, target);
        }
    }
}

fn part_1() {
    let rules = get_rules("data/test.txt", Mode::CanBeContained);
    let mut reachable_colors = HashSet::new();
    let mut to_search = VecDeque::new();
    to_search.push_front("shiny_gold".to_string());
    let mut visited = HashSet::new();
    while let Some(target) = to_search.pop_front() {
        visited.insert(target.clone());
        if let Some(source) = rules.get(&target) {
            for (_qty, color) in source {
                reachable_colors.insert(color.clone());
                if !visited.contains(color) {
                    to_search.push_back(color.clone());
                }
            }
        }
    }
    println!("PART 1: reachable_colors: {:?}", reachable_colors.len());
}

fn part_2() {
    let rules = get_rules("data/input1.txt", Mode::MustContain);

    let mut bags_that_fit = Vec::new();
    let mut to_search = VecDeque::new();
    to_search.push_back((1, "shiny_gold".to_string()));

    while let Some((multiplier, target)) = to_search.pop_front() {
        if let Some(source) = rules.get(&target) {
            for (qty, color) in source {
                bags_that_fit.push((qty * multiplier, color.clone()));
                to_search.push_back((qty * multiplier, color.clone()));
            }
        } else {
            bags_that_fit.push((multiplier, target));
        }
    }
    // println!("PART 2: bags_that_fit: {:#?}", bags_that_fit);
    println!(
        "PART 2: bags_that_fit: {:#?}",
        bags_that_fit.iter().map(|(x, _)| x).sum::<usize>()
    );
}
fn main() {
    part_1();
    part_2();
}
