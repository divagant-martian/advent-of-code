use std::fs::read_to_string;

pub fn get_data_from_str(string: &str) -> Vec<usize> {
    string
        .split_ascii_whitespace()
        .map(|x| usize::from_str_radix(x, 10).unwrap())
        .collect()
}

fn main() {
    println!("{:-<50}", "- PART 1 -");
    let numbers = get_data_from_str(&read_to_string("data/input1.txt").expect("bad input"));
    let mut seen = [false; 2020]; // all nums are less than 2020
    for num in &numbers {
        seen[*num] = true;
    }
    let ans = numbers.iter().find(|&n| seen[2020 - *n]).unwrap();
    println!("{:?}", ans * (2020 - ans));

    println!("{:-<50}", "- PART 2 -");
    for n0 in &numbers {
        for n1 in &numbers {
            let n2 = n0 + n1;
            if n2 < 2020 {
                if seen[2020 - n2] {
                    let n2 = 2020 - n2;
                    println!("{:?}, {}", (n0, n1, n2), n0 * n1 * n2, );
                    return;
                }
            }
        }
    }
}
