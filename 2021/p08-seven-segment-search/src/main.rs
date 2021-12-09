mod digit;
mod guess;
mod parsing;
mod signal;
mod signal_set;

fn main() {
    let input = std::fs::read_to_string("data/input.txt").expect("Input is ok");
    let ans1 = parsing::parse_input(&input)
        .into_iter()
        .flat_map(|(clues, outs)| {
            let mappings = guess::corrupt_digit_signals(clues);
            let digits = guess::decipher(outs, mappings);
            digits
        })
        .filter(|n| [1, 4, 7, 8].contains(n))
        .count();
    println!("First one: {}", ans1);

    let ans2 = parsing::parse_input(&input)
        .into_iter()
        .map(|(clues, outs)| {
            let mappings = guess::corrupt_digit_signals(clues);
            let digits = guess::decipher(outs, mappings);
            digits
                .into_iter()
                .rev()
                .enumerate()
                .map(|(i, n)| {
                    let num: u32 = 10u32.pow(i as u32) * (n as u32);
                    num
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("{:?}", ans2);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_first_line() {
        let (clues, outs) = parsing::parse_line(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let mappings = guess::corrupt_digit_signals(clues);
        let digits = guess::decipher(outs, mappings);
        assert_eq!(digits, [5, 3, 5, 3]);
    }

    #[test]
    fn whole_first_example() {
        let input = "
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ";
        let ans1 = parsing::parse_input(input)
            .into_iter()
            .flat_map(|(clues, outs)| {
                let mappings = guess::corrupt_digit_signals(clues);
                let digits = guess::decipher(outs, mappings);
                digits
            })
            .filter(|n| [1, 4, 7, 8].contains(n))
            .count();
        assert_eq!(ans1, 26);
    }

    #[test]
    fn whole_second_example() {
        let input = "
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ";
        let ans1 = parsing::parse_input(input)
            .into_iter()
            .map(|(clues, outs)| {
                let mappings = guess::corrupt_digit_signals(clues);
                let digits = guess::decipher(outs, mappings);
                digits
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(|(i, n)| {
                        let num: u32 = 10u32.pow(i as u32) * (n as u32);
                        num
                    })
                    .sum::<u32>()
            })
            .sum::<u32>();
        assert_eq!(ans1, 61229);
    }
}
