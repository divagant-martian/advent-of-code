use std::collections::HashMap;

fn main() {
    let nums = "0,12,6,13,20,1,17";
    let mut last_turns = HashMap::new();
    let mut last_said = 0;
    for (i, n) in nums.split(",").enumerate() {
        let n = n.parse::<usize>().unwrap();
        last_turns.insert(n, [i + 1, 0]);
        last_said = n;
    }

    let n = last_turns.len();
    for turn in n + 1..=30000000 {
        last_said = if let Some(lasts) = last_turns.get(&last_said) {
            if lasts[1] == 0 {
                0
            } else {
                lasts[0] - lasts[1]
            }
        } else {
            0
        };

        // println!("turn {} say {}", turn, last_said);

        if let Some(lasts) = last_turns.get_mut(&last_said) {
            lasts[1] = lasts[0];
            lasts[0] = turn;

        } else {
            last_turns.insert(last_said, [turn, 0]);
        }

    }

    println!("last said: {}", last_said);
}
