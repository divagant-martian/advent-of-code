use crate::signal_set::SignalSet;

pub fn parse_line(input: &str) -> ([SignalSet; 10], [SignalSet; 4]) {
    let (clues, outs) = input
        .trim()
        .split_once(" | ")
        .expect("ins and outs are present");
    let clues = clues
        .trim()
        .split_whitespace()
        .map(SignalSet::from)
        .collect::<Vec<_>>()
        .try_into()
        .expect("10 clues");
    let outs = outs
        .trim()
        .split_whitespace()
        .map(SignalSet::from)
        .collect::<Vec<_>>()
        .try_into()
        .expect("4 outs");
    (clues, outs)
}

pub fn parse_input(input: &str) -> Vec<([SignalSet; 10], [SignalSet; 4])> {
    input.trim().lines().map(parse_line).collect()
}
