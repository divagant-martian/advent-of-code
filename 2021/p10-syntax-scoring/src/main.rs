use syntax::Left;
use syntax::Right;
use syntax::Syntax;

mod syntax;

#[derive(Debug, PartialEq, Eq)]
enum SyntaxErr {
    Incomplete { missing: Vec<Right> },
    Excess(Right),
    Corrupt { expected: Right, found: Right },
}

impl SyntaxErr {
    fn score(&self) -> usize {
        match self {
            SyntaxErr::Incomplete { missing } => missing.iter().fold(0, |acc, r| {
                acc * 5
                    + match r {
                        Right::A => 1,
                        Right::B => 2,
                        Right::C => 3,
                        Right::D => 4,
                    }
            }),
            SyntaxErr::Excess(_) => todo!(),
            SyntaxErr::Corrupt { expected: _, found } => match found {
                Right::A => 3,
                Right::B => 57,
                Right::C => 1197,
                Right::D => 25137,
            },
        }
    }
}

fn verify(line: &[Syntax]) -> Result<(), SyntaxErr> {
    let mut queue = Vec::default();
    for s in line {
        match s {
            Syntax::Open(left) => queue.push(left),
            Syntax::Close(right) => match queue.pop() {
                Some(left) => {
                    let expected = left.close();
                    if expected != *right {
                        return Err(SyntaxErr::Corrupt {
                            expected,
                            found: *right,
                        });
                    }
                }
                None => return Err(SyntaxErr::Excess(*right)),
            },
        }
    }

    let missing: Vec<Right> = queue.into_iter().map(Left::close).rev().collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(SyntaxErr::Incomplete { missing })
    }
}

fn main() {
    let input = std::fs::read_to_string("data/input.txt").expect("Input is ok.");
    let mut corrupt_score = 0;
    let mut incomplete_scores = Vec::default();
    for line in input.lines() {
        let syntax = line
            .chars()
            .map(Syntax::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("input is ok");

        let verify_result = verify(&syntax);
        if let Err(ref e) = verify_result {
            match e {
                SyntaxErr::Incomplete { .. } => incomplete_scores.push(e.score()),
                SyntaxErr::Excess(_) => todo!(),
                SyntaxErr::Corrupt { .. } => corrupt_score += e.score(),
            }
        }
    }
    println!("Error score: {}", corrupt_score);
    incomplete_scores.sort();
    println!(
        "Middle score: {}",
        incomplete_scores[incomplete_scores.len() / 2]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_ok() {
        let input = "[<>({}){}[([])<>]]";
        let syntax = input
            .chars()
            .map(Syntax::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("input is ok");
        assert_eq!(verify(&syntax), Ok(()));
    }

    #[test]
    fn test_verify_corrupt() {
        let right_pa = ')'.try_into().unwrap();
        let right_sq = ']'.try_into().unwrap();
        let right_br = '}'.try_into().unwrap();
        let right_tr = '>'.try_into().unwrap();

        let test_cases = [
            ("{([(<{}[<>[]}>{[]{[(<()>", right_sq, right_br),
            ("[[<[([]))<([[{}[[()]]]", right_sq, right_pa),
            ("[{[{({}]{}}([{[{{{}}([]", right_pa, right_sq),
            ("[<(<(<(<{}))><([]([]()", right_tr, right_pa),
            ("<{([([[(<>()){}]>(<<{{", right_sq, right_tr),
        ];
        let mut total_score = 0;
        for (input, expected, found) in test_cases {
            let syntax = input
                .chars()
                .map(Syntax::try_from)
                .collect::<Result<Vec<_>, _>>()
                .expect("input is ok");

            let verify_result = verify(&syntax);
            total_score += match verify_result.as_ref() {
                Err(e) => e.score(),
                _ => 0,
            };

            assert_eq!(
                verify_result,
                Err(SyntaxErr::Corrupt { expected, found }),
                "{}",
                input
            );
        }

        assert_eq!(total_score, 26397);
    }

    #[test]
    fn test_verify_incomplete() {
        let test_cases = [
            ("[({(<(())[]>[[{[]{<()<>>", "}}]])})]", 288957),
            ("[(()[<>])]({[<{<<[]>>(", ")}>]})", 5566),
            ("(((({<>}<{<{<>}{[]{[]{}", "}}>}>))))", 1480781),
            ("{<[[]]>}<{[{[{[]{()[[[]", "]]}}]}]}>", 995444),
            ("<{([{{}}[<[[[<>{}]]]>[]]", "])}>", 294),
        ];
        for (input, expected_missing, expected_score) in test_cases {
            let syntax = input
                .chars()
                .map(Syntax::try_from)
                .collect::<Result<Vec<_>, _>>()
                .expect("input is ok");

            let expected_missing = expected_missing
                .chars()
                .map(Right::try_from)
                .collect::<Result<Vec<_>, _>>()
                .expect("missing is ok");

            match verify(&syntax) {
                Err(e) => {
                    assert_eq!(
                        e,
                        SyntaxErr::Incomplete {
                            missing: expected_missing
                        },
                        "{}",
                        input,
                    );
                    assert_eq!(e.score(), expected_score, "{}", input);
                }
                _ => panic!("wrong result"),
            }
        }
    }
}
