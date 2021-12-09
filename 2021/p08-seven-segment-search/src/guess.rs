use std::collections::HashMap;

use crate::{digit::Digit, signal::Signal, signal_set::SignalSet};

pub fn corrupt_digit_signals(clues: [SignalSet; 10]) -> HashMap<SignalSet, Digit> {
    let one_in = *clues.iter().find(|set| set.len() == 2).expect("1 is there");
    let seven_in = *clues.iter().find(|set| set.len() == 3).expect("7 is there");

    let a_in = seven_in - one_in;
    let a_in = a_in.iter().next().unwrap();

    let sixes = clues
        .iter()
        .filter(|set| set.len() == 6)
        .map(|&set| !set)
        .reduce(|set_a, set_b| set_a | set_b)
        .expect("Non empty");

    let twos_signal = clues
        .iter()
        .find(|set| set.len() == 5 && set.contains(a_in) && set.is_superset(sixes))
        .cloned()
        .expect("Non empty");

    let g_in = twos_signal
        .iter()
        .find(|&s| !sixes.contains(s) && s != a_in)
        .expect("it exists");

    let threes_signal = *clues
        .iter()
        .find(|set| set.len() == 5 && set.contains(g_in) && set.is_superset(seven_in))
        .expect("Non empty");

    let d_in = threes_signal
        .iter()
        .find(|&s| !seven_in.contains(s) && s != g_in)
        .expect("it exists");

    let fives_signal = clues
        .iter()
        .find(|&&set| set.len() == 5 && set != twos_signal && set != threes_signal)
        .cloned()
        .expect("Non empty");

    let mut b_and_f_in = fives_signal;
    b_and_f_in.remove(a_in);
    b_and_f_in.remove(g_in);
    b_and_f_in.remove(d_in);

    let f_in = b_and_f_in
        .iter()
        .find(|&s| one_in.contains(s))
        .expect("f is in ");

    let b_in = b_and_f_in
        .iter()
        .find(|&s| !one_in.contains(s))
        .expect("b is in ");

    let c_in = one_in
        .iter()
        .find(|&s| s != f_in)
        .expect("one is made of c and f");

    let e_in = (!SignalSet::from([a_in, b_in, c_in, d_in, f_in, g_in].as_slice()))
        .iter()
        .next()
        .expect("all good");

    Digit::ALL
        .into_iter()
        .map(|d| {
            let mut corrupted = SignalSet::empty();
            for correct_s in d.signals() {
                match correct_s {
                    Signal::A => corrupted.insert(a_in),
                    Signal::B => corrupted.insert(b_in),
                    Signal::C => corrupted.insert(c_in),
                    Signal::D => corrupted.insert(d_in),
                    Signal::E => corrupted.insert(e_in),
                    Signal::F => corrupted.insert(f_in),
                    Signal::G => corrupted.insert(g_in),
                }
            }
            (corrupted, *d)
        })
        .collect::<std::collections::HashMap<_, _>>()
}

pub fn decipher(outs: [SignalSet; 4], mappings: HashMap<SignalSet, Digit>) -> [u8; 4] {
    outs.map(|set| mappings[&set].as_u8())
}
