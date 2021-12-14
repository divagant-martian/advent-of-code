use std::collections::HashMap;
type Template = HashMap<(char, char), usize>;
type Rules = HashMap<(char, char), char>;

fn parse_template(input: &str) -> Template {
    let mut counts = HashMap::with_capacity(input.len() * 2);
    let mut prev = None;
    for c in input.chars() {
        if let Some(prev) = prev.take() {
            *counts.entry((prev, c)).or_default() += 1;
        }
        prev = Some(c);
    }
    let last = prev.expect("Should not be empty");
    *counts.entry((last, ' ')).or_default() += 1;
    counts
}

fn parse(input: &str) -> Result<(Template, Rules), &'static str> {
    let (template, insertion_rules) = input
        .trim()
        .split_once("\n\n")
        .ok_or("Bad format, expected a two part input")?;
    let template = parse_template(template);
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
        .collect::<Result<Rules, _>>()?;
    Ok((template, insertion_rules))
}

fn apply(template: &mut Template, rules: &Rules) {
    let values = template.drain().collect::<Vec<_>>();
    for (chars_in, count) in values {
        let (first, second) = chars_in;
        if let Some(&char_out) = rules.get(&chars_in) {
            *template.entry((first, char_out)).or_default() += count;
            *template.entry((char_out, second)).or_default() += count;
        } else {
            template.insert((first, second), count);
        }
    }
}

fn ans(template: &Template) -> usize {
    let mut max = 0;
    let mut min = usize::max_value();
    let mut counts = HashMap::<char, usize>::new();
    for (&(first, _), count) in template {
        *counts.entry(first).or_default() += count;
    }
    for v in counts.values() {
        max = *v.max(&max);
        min = *v.min(&min);
    }
    max - min
}

fn main() {
    let input = std::fs::read_to_string("data/input").expect("Input is present");
    let (mut template, rules) = parse(&input).expect("Input is ok");
    for _ in 0..10 {
        apply(&mut template, &rules);
    }
    println!("{}", ans(&template));

    for step in 10..40 {
        println!("Running step {}", step);
        apply(&mut template, &rules);
    }
    println!("{}", ans(&template));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
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
        CN -> C";

    #[test]
    fn test_parse() {
        let (_template, rules) = parse(EXAMPLE).unwrap();

        assert_eq!(rules.len(), 16);
    }

    #[test]
    fn test_apply() {
        let (mut template, rules) = parse(EXAMPLE).unwrap();

        for (step, expected) in [
            "NCNBCHB",
            "NBCCNBBBCBHCB",
            "NBBBCNCCNBBNBNBBCHBHHBCHB",
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
        ]
        .into_iter()
        .enumerate()
        {
            apply(&mut template, &rules);
            assert_eq!(
                template,
                parse_template(expected),
                "Step {} should be ok",
                step
            );
        }
    }

    #[test]
    fn test_counts() {
        let (mut template, rules) = parse(EXAMPLE).unwrap();
        // After step 5, it has length 97; After step 10, it has length 3073
        for step in 1..=10 {
            apply(&mut template, &rules);
            if step == 5 {
                assert_eq!(template.values().sum::<usize>(), 97);
            }
        }
        assert_eq!(template.values().sum::<usize>(), 3073);
        let ans = ans(&template);
        assert_eq!(ans, 1588);
    }
}
