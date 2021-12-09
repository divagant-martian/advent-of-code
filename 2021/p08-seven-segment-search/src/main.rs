mod digit;
mod parsing;
mod signal;

use digit::Digit::*;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input =
            parsing::parse_clues("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab");
    }
}
