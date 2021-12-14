use std::collections::HashMap;

fn parse(input: &str) -> Result<(Vec<char>, HashMap<(char, char), char>), &'static str> {
    let (template, insertion_rules) = input
        .trim()
        .split_once("\n\n")
        .ok_or("Bad format, expected a two part input")?;
    let template = template.chars().collect();
    let insertion_rules = insertion_rules
        .lines()
        .map(|l| {
            l.trim()
                .split_once(" -> ")
                .map(|(matching, result)| {
                    let mut chars = matching.chars();
                    let first = chars.next().ok_or("Empty lhs of insertion rule")?;
                    let second = chars
                        .next()
                        .ok_or("Missing second part of lhs of insertion rule")?;
                    if chars.next().is_some() {
                        return Err("lhs of insertion rule has too many parts");
                    }
                    let result = if result.len() == 1 {
                        result.chars().next().ok_or("Empty rhs of insertion rule")
                    } else {
                        Err("Too many parts in rhs of insertion rule")
                    }?;
                    Ok(((first, second), result))
                })
                .ok_or("Bad format. Missing ' -> ' in insertion rule")?
        })
        .collect::<Result<HashMap<_, _>, _>>()?;
    Ok((template, insertion_rules))
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (template, rules) = parse(
            "
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C",
        )
        .unwrap();

        assert_eq!(&template, &['N', 'N', 'C', 'B']);
        assert_eq!(rules.len(), 16);
    }
}
