use std::collections::HashMap;
type Cup = usize;

fn wrapping_dec(me: usize, n: usize) -> usize {
    me.wrapping_sub(2).min(n - 1) + 1
}

fn main() {
    // Get the parameters: starting sequence and number of rounds
    let nums = std::env::args()
        .nth(1)
        .expect("Provide a sequence of numbers (like 389125467)");
    let rounds: usize = std::env::args()
        .nth(2)
        .expect("Provide a number of times to shift the cups")
        .parse()
        .unwrap();

    // Store the item and the one next to it in a hashmap
    let mut danums = HashMap::with_capacity(1_000_000);
    let mut nums: Vec<Cup> = nums
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect();
    let mut n = nums.len();
    while nums.len() < 1_000_000 {
        nums.push(n + 1);
        n += 1;
    }
    for w in nums.windows(2) {
        danums.insert(w[0], w[1]);
    }
    // add the last
    danums.insert(nums[n - 1], nums[0]);

    // get the first cup
    let mut current_cup = nums[0];

    for round in 1..=rounds {
        if round % 1_000_000 == 0 {
            println!("Round {}", round);
        }
        let mut picked_cups = Vec::new();
        for _ in 0..3 {
            // remove the current's next
            let to_remove = danums[&current_cup];
            let next = danums.remove(&to_remove).unwrap();
            *danums.get_mut(&current_cup).unwrap() = next;
            picked_cups.push(to_remove);
        }

        let mut destination_cup = wrapping_dec(current_cup, n);

        while picked_cups.contains(&destination_cup) {
            destination_cup = wrapping_dec(destination_cup, n);
        }

        // add them where they belong
        for item in picked_cups.into_iter().rev() {
            // insert with the appropriate prev and next
            let next = danums[&destination_cup];
            danums.insert(item, next);
            // update the next of prev
            *danums.get_mut(&destination_cup).unwrap() = item;
        }

        // update current_cup
        current_cup = danums[&current_cup];
    }

    let next = danums[&1];
    println!(
        "next {:?} nextnext {} mult {}",
        next,
        danums[&next],
        danums[&next] * next
    );
}
