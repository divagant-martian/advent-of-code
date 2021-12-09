use std::collections::HashSet;

use crate::{
    digit::Digit::{self, *},
    signal::Signal,
};

pub fn possible_signal_matchings(signal_set: &Vec<Signal>) -> HashSet<Signal> {
    digits_with_len(signal_set.len())
        .into_iter()
        .flat_map(Digit::signals)
        .cloned()
        .collect()
}

pub const fn digits_with_len(len: usize) -> &'static [Digit] {
    match len {
        2 => &[D1],
        3 => &[D7],
        4 => &[D4],
        5 => &[D2, D3, D5],
        6 => &[D0, D6, D9],
        7 => &[D8],
        _ => panic!("wtf"),
    }
}


