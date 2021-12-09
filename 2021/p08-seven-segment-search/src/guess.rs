use std::collections::{BTreeSet, HashSet};

use crate::{
    digit::Digit::{self, *},
    signal::{self, Signal},
};

pub fn possible_signal_matchings(signal_set: &BTreeSet<Signal>) -> HashSet<Signal> {
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

pub fn guess(clues: [BTreeSet<Signal>; 10]) {
    let one = clues
        .iter()
        .find_map(|set| {
            (set.len() == 2).then(|| {
                (
                    set.clone().into_iter().collect::<BTreeSet<_>>(),
                    possible_signal_matchings(set),
                )
            })
        })
        .expect("1 is there");

    let seven = clues
        .iter()
        .find_map(|set| {
            (set.len() == 3).then(|| {
                (
                    set.clone().into_iter().collect::<BTreeSet<_>>(),
                    possible_signal_matchings(set),
                )
            })
        })
        .expect("7 is there");

    let four = clues
        .iter()
        .find_map(|set| {
            (set.len() == 4).then(|| {
                (
                    set.clone().into_iter().collect::<BTreeSet<_>>(),
                    possible_signal_matchings(set),
                )
            })
        })
        .expect("4 is there");

    println!("{:?}", one);
    println!("{:?}", seven);
    println!("{:?}", four);

    let (seven_in, seven_out) = seven;
    let (one_in, one_out) = one;
    let a_in: BTreeSet<_> = seven_in.difference(&one_in).cloned().collect();
    let a_out: BTreeSet<_> = seven_out.difference(&one_out).cloned().collect();
    assert_eq!(a_in.len(), 1);
    assert_eq!(a_out.len(), 1);
    assert_eq!(a_out.into_iter().next(), Some(Signal::A));
    let a_in = a_in.into_iter().next().unwrap();
    println!("{:?} -> A", a_in);
    // let (one_in, one_out) = one;
    //
    let sixes = clues
        .iter()
        .filter(|set| set.len() == 6)
        .map(|set| {
            Signal::ALL
                .iter()
                .filter(|s| !set.contains(s))
                .cloned()
                .collect::<BTreeSet<_>>()
        })
        .reduce(|set_a, set_b| set_a.union(&set_b).into_iter().cloned().collect())
        .expect("Non empty");
    println!("{:?} > {{C, D, E}}", sixes);

    let twos_signal = clues
        .iter()
        .find(|set| set.len() == 5 && set.contains(&a_in) && set.is_superset(&sixes))
        .cloned()
        .expect("Non empty");

    let g_in = twos_signal
        .iter()
        .find(|&s| !sixes.contains(s) && s != &a_in)
        .cloned()
        .expect("it exists");

    println!("{:?} -> G", g_in);

    let threes_signal = clues
        .iter()
        .find(|set| set.len() == 5 && set.contains(&g_in) && set.is_superset(&seven_in))
        .cloned()
        .expect("Non empty");

    let d_in = threes_signal
        .iter()
        .find(|&s| !seven_in.contains(s) && s != &g_in)
        .cloned()
        .expect("it exists");

    println!("{:?} -> D", d_in);

    let fives_signal = clues
        .iter()
        .find(|&set| set.len() == 5 && set != &twos_signal && set != &threes_signal)
        .cloned()
        .expect("Non empty");

    let b_and_f_in = fives_signal
        .iter()
        .filter(|&&s| s != a_in && s != g_in && s != d_in)
        .cloned()
        .collect::<BTreeSet<_>>();

    println!("{:?} -> {{B, F}}", b_and_f_in);
    let f_in = b_and_f_in
        .iter()
        .find(|s| one_in.contains(s))
        .cloned()
        .expect("your mom");
    println!("{:?} -> {{F}}", f_in);

    let b_in = b_and_f_in
        .iter()
        .find(|s| !one_in.contains(s))
        .cloned()
        .expect("your mom");
    println!("{:?} -> {{B}}", b_in);

    let c_in = one_in
        .iter()
        .find(|&s| s != &f_in)
        .expect("one is made of c and f")
        .clone();
    println!("{:?} -> {{C}}", c_in);

    let e_in = Signal::ALL
        .into_iter()
        .find(|s| !&[a_in, b_in, c_in, d_in, f_in, g_in].contains(s)).expect("AAA");
    println!("{:?} -> {{E}}", e_in);
    // signal::print_signals(&threes_signal.into_iter().collect::<Vec<_>>());
    // let fives_union = fives
}
