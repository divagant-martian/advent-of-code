use std::fs::read_to_string;

enum Policy {
    Part1,
    Part2,
}

fn count_valid(string: &str, policy: Policy) -> usize {
    string
        .lines()
        .filter(|line| is_line_valid(line, &policy))
        .count()
}

fn is_line_valid<'a>(linea: &'a str, policy: &Policy) -> bool {
    let line = linea
        .split(|c| c == ' ' || c == '-' || c == ':')
        .collect::<Vec<_>>();
    let first_num: usize = line[0].parse().unwrap();
    let second_num: usize = line[1].parse().unwrap();
    let letter = line[2].chars().next().unwrap();
    let pwd = line[4];
    match policy {
        Policy::Part1 => {
            let count = pwd.chars().filter(|c| c == &letter).count();
            first_num <= count && count <= second_num
        }
        Policy::Part2 => {
            let valid = pwd
                .chars()
                .fold((false, 1), |(acc, idx), c| {
                    let is_valid_alone = (idx == first_num || idx == second_num) && c == letter;
                    (
                        if !acc {
                            is_valid_alone
                        } else {
                            !is_valid_alone
                        },
                        idx + 1,
                    )
                })
                .0;
            valid
        }
    }
}

fn main() {
    let data = &read_to_string("data/input1.txt").expect("bad input");

    println!("{:-<50}", "- PART 1 -");
    let valid = count_valid(data, Policy::Part1);
    println!("{:?}", valid);

    println!("{:-<50}", "- PART 2 -");
    let valid = count_valid(data, Policy::Part2);
    println!("{:?}", valid);
}
