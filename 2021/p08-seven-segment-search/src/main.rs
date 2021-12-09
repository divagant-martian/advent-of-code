mod digit;
mod guess;
mod parsing;
mod signal;
mod signal_set;

fn main() {
    let (clues, outs) = parsing::parse_input(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
    )[0];
    let mappings = guess::corrupt_digit_signals(clues);
    let digits = guess::decipher(outs, mappings);
    assert_eq!(digits, [5, 3, 5, 3]);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_possible_matchings() {
        let (clues, outs) = parsing::parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let mappings = guess::corrupt_digit_signals(clues);
        let digits = guess::decipher(outs, mappings);
        assert_eq!(digits, [5, 3, 5, 3]);
    }
}
