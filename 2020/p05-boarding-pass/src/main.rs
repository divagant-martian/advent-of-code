use std::fs::read_to_string;

fn main() {
    let data = read_to_string("data/input1.txt").expect("bad input");
    let mut max_id = 0;
    let mut ids = Vec::new();
    for pass in data.lines() {
        let replaced = pass
            .chars()
            .map(|c| match c {
                'B' | 'R' => '1',
                'F' | 'L' => '0',
                _ => unreachable!(),
            })
            .collect::<String>();
        let id = usize::from_str_radix(&replaced, 2).unwrap();
        let idx = ids.binary_search(&id).unwrap_or_else(|x| x);
        ids.insert(idx, id);
        max_id = max_id.max(id);
    }

    for window in ids.windows(2) {
        if window[0] + 2 == window[1] {
            println!("{}", window[0] + 1)
        }
    }

    println!("Answer 1 {}", max_id);
}
