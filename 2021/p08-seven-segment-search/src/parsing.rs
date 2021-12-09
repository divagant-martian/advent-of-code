use crate::signal::Signal;

pub fn parse_clues(input: &str) -> [Vec<Signal>; 10] {
    input
        .trim()
        .split_whitespace()
        .map(|word| word.chars().map(Signal::from).collect())
        .collect::<Vec<_>>()
        .try_into()
        .expect("10 clues")
}
