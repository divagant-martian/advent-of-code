use std::collections::HashMap;
type Cup = usize;

fn paint_cups(cups: &HashMap<usize, (usize, usize)>, current_cup: usize) {
    let (stop, mut next) = cups[&current_cup];
    let sep = ", ";
    print!("{}{}({})", stop, sep, current_cup);
    while next != stop {
        print!("{}{}", sep, next);
        next = cups[&next].1;
    }
    println!()
}

fn wrapping_dec(me: usize, n: usize) -> usize {
    me.wrapping_sub(2).min(n - 1) + 1
}

fn main() {
    let nums = std::env::args()
        .nth(1)
        .expect("Provide a sequence of numbers (like 389125467)");
    let rounds: usize = std::env::args()
        .nth(2)
        .expect("Provide a number of times to shift the cups")
        .parse()
        .unwrap();
    let mut nums: Vec<Cup> = nums
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect();
    let mut n = nums.len();
    while nums.len() < 1_000_000 {
        nums.push(n + 1);
        n += 1;
    }
    let mut danums = HashMap::with_capacity(n);
    for w in nums.windows(3) {
        danums.insert(w[1], (w[0], w[2]));
    }
    // add the first and last
    danums.insert(nums[0], (nums[n - 1], nums[1]));
    danums.insert(nums[n - 1], (nums[n - 2], nums[0]));

    let mut current_cup = nums[0];

    for round in 1..=rounds {
        // if round % 1_000_000 == 0 {
        // println!("Round {}", round);
        // paint_cups(&danums, current_cup);
        // }
        let mut picked_cups = Vec::new();
        for _ in 0..3 {
            // remove the current's next
            let to_remove = danums[&current_cup].1;
            let (prev, next) = danums.remove(&to_remove).unwrap();
            picked_cups.push(to_remove);
            danums.get_mut(&prev).expect("prev exists").1 = next;
            danums.get_mut(&next).expect("next exists").0 = prev;
        }
        // print!("Pick up: {:?} ", picked_cups);

        let mut destination_cup = wrapping_dec(current_cup, n);

        while picked_cups.contains(&destination_cup) {
            destination_cup = wrapping_dec(destination_cup, n);
        }
        // println!(
        // "current cup {} destination cup {:?}",
        // current_cup, destination_cup
        // );

        // println!("destination position {:?}", destination_position);
        // add them where they belong
        for item in picked_cups.into_iter().rev() {
            // insert with the appropriate prev and next
            let next = danums[&destination_cup].1;
            danums.insert(item, (destination_cup, next));
            // update the prev of next and next of prev
            danums.get_mut(&next).unwrap().0 = item;
            danums.get_mut(&destination_cup).unwrap().1 = item;
        }

        // update current_cup
        current_cup = danums[&current_cup].1;
    }
    let next = danums[&1].1;
    println!(
        "next {:?} nextnext {} mult {}",
        next,
        danums[&next].1,
        danums[&next].1 * next
    );
}
