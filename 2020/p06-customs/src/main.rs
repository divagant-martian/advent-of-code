use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let mut groups_answers = HashMap::with_capacity(27);
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    let mut group_persons = 0;
    for group in read_to_string("data/input1.txt")
        .expect("bad input")
        .split("\n\n")
    {
        for person in group.split_ascii_whitespace() {
            for ans in person.chars() {
                *groups_answers.entry(ans).or_insert(0_usize) += 1;
            }
            group_persons += 1;
        }
        total_p1 += groups_answers.len();
        total_p2 += groups_answers
            .values()
            .filter(|&count| *count == group_persons)
            .count();
        groups_answers.clear();
        group_persons = 0;
    }

    println!("answer 1: {}", total_p1);
    println!("answer 2: {}", total_p2);
}
