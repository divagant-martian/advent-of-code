fn main() {
    let report: Vec<_> = std::fs::read_to_string("data/input.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect();

    let bit_counts = bit_counts::<12>(&report);
    let (gama, epsilon) = rates(&bit_counts);
    let gama = usize::from_str_radix(&format_line(&gama), 2).unwrap();
    let epsilon = usize::from_str_radix(&format_line(&epsilon), 2).unwrap();
    println!("{}", gama * epsilon);

    let oxigen_raiting =
        usize::from_str_radix(&format_line(&rate_filter::<12>(&report, Bit::I)), 2).unwrap();
    let co2_raiting =
        usize::from_str_radix(&format_line(&rate_filter::<12>(&report, Bit::O)), 2).unwrap();
    println!(
        "oxigen {} co2 {} life_support {}",
        oxigen_raiting,
        co2_raiting,
        oxigen_raiting * co2_raiting
    );
}

/// Bit representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    O,
    I,
}

/// Count how many times each bit appears in each position.
fn bit_counts<const N: usize>(
    report: &[Vec<Bit>],
) -> [(usize /* count of 1s */, usize /* count of 0s */); N] {
    report.into_iter().fold([(0, 0); N], |mut counter, line| {
        for (i, c) in line.iter().enumerate() {
            match c {
                Bit::O => counter[i].1 += 1,
                Bit::I => counter[i].0 += 1,
            }
        }
        return counter;
    })
}

/// Get the gama and epsilon rates.
fn rates<const N: usize>(counts: &[(usize, usize); N]) -> ([Bit; N], [Bit; N]) {
    let mut gama = [Bit::O; N];
    let mut epsilon = [Bit::I; N];
    for (i, (ones, zeros)) in counts.into_iter().enumerate() {
        if ones >= zeros {
            gama[i] = Bit::I;
            epsilon[i] = Bit::O;
        }
    }
    (gama, epsilon)
}

fn rate_filter<const N: usize>(report: &[Vec<Bit>], preference: Bit) -> Vec<Bit> {
    let mut options: Vec<_> = report.iter().collect();
    for i in 0..N {
        let (ones, zeros) = options.iter().fold((0, 0), |mut counter, bitvec| {
            if bitvec[i] == Bit::I {
                counter.0 += 1;
                counter
            } else {
                counter.1 += 1;
                counter
            }
        });
        let searched_bit = match preference {
            Bit::O => {
                // searching the least common
                if zeros <= ones {
                    Bit::O
                } else {
                    Bit::I
                }
            }
            Bit::I => {
                // searching the most common
                if ones >= zeros {
                    Bit::I
                } else {
                    Bit::O
                }
            }
        };
        options = options
            .into_iter()
            .filter(|bitvec| &bitvec[i] == &searched_bit)
            .collect();
        println!(
            "Remaining options after filtering bit {} with {:?}, {:?}",
            i, searched_bit, options
        );
        if options.len() == 1 {
            return options[0].to_vec();
        }
    }
    panic!("Not found")
}

impl std::convert::From<char> for Bit {
    fn from(c: char) -> Self {
        match c {
            '0' => Bit::O,
            '1' => Bit::I,
            other => panic!("Unexpected char {}", other),
        }
    }
}

/// Parse a line of 0s and 1s as a vector of Bit.
fn parse_line(line: &str) -> Vec<Bit> {
    line.chars().map(Bit::from).collect()
}

fn format_line(bits: &[Bit]) -> String {
    bits.into_iter()
        .map(|b| match b {
            Bit::I => '1',
            Bit::O => '0',
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let expected_gama = parse_line("10110");
        let expected_epsilon = parse_line("01001");

        let expected_gama_dec = 22;
        let expected_epsilon_dec = 9;

        let report: Vec<_> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(parse_line)
        .collect();

        let bit_counts = bit_counts::<5>(&report);
        println!("{:?}", bit_counts);
        let (gama, epsilon) = rates(&bit_counts);
        assert_eq!(gama.as_slice(), expected_gama.as_slice());
        assert_eq!(epsilon.as_slice(), expected_epsilon.as_slice());
        let dec_gama = format_line(&gama);
        let dec_epsilon = format_line(&epsilon);
        assert_eq!(usize::from_str_radix(&dec_gama, 2), Ok(expected_gama_dec));
        assert_eq!(
            usize::from_str_radix(&dec_epsilon, 2),
            Ok(expected_epsilon_dec)
        );

        let oxigen_raiting = rate_filter::<5>(&report, Bit::I);
        let expected_oxigen = parse_line("10111");
        assert_eq!(oxigen_raiting, expected_oxigen);

        let co2_raiting = rate_filter::<5>(&report, Bit::O);
        let expected_co2 = parse_line("01010");
        assert_eq!(expected_co2, co2_raiting);
    }
}
