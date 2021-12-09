mod digit;
mod guess;
mod parsing;
mod signal;

use digit::Digit::*;
use signal::Signal::{self, *};

fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashMap, HashSet};

    use super::*;

    #[test]
    fn test_possible_matchings() {
        let input =
            parsing::parse_clues("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");

        let expected_matchings = [
            Signal::ALL,
            Signal::ALL,
            Signal::ALL,
            Signal::ALL,
            &[A, C, F],
            Signal::ALL,
            Signal::ALL,
            &[B, C, D, F],
            Signal::ALL,
            &[C, F],
        ]
        .map(|array| array.iter().cloned().collect::<HashSet<_>>());

        for (clue, expected) in input.iter().zip(expected_matchings.iter()) {
            assert_eq!(expected, &guess::possible_signal_matchings(clue))
        }

        let mut matchings = input
            .into_iter()
            .map(|x| {
                let guesses = guess::possible_signal_matchings(&x);
                let clue = x.into_iter().collect::<BTreeSet<_>>();
                (clue, guesses)
            })
            .collect::<Vec<_>>();

        let mut used_matchings = Vec::new();

        for iteration in 0..10 {
            let (pos, clue, guess) = matchings
                .iter()
                .enumerate()
                .min_by_key(|(_i, (_clue, guess))| guess.len())
                .map(|(i, (clue, guess))| (i, clue.clone(), guess.clone()))
                .expect("sd");

            used_matchings.push(matchings.remove(pos));

            for (c, g) in matchings.iter_mut() {
                if c.is_superset(&clue) && c != &clue {
                    *c = c.difference(&clue).cloned().collect();
                    assert!(g.is_superset(&guess), "chris is right");
                    *g = g.difference(&guess).cloned().collect();
                }
            }

            println!(
                "[{}]\n{:?}\nused: {:?}\n",
                iteration, matchings, used_matchings
            );
        }
    }
}
