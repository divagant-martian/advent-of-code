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
        guess::guess(input);
        // fa
        // fg
        // ag
    }
}
