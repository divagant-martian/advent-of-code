use std::fs::read_to_string;

type Num = u8;
fn load_sorted(filename: &'static str) -> Vec<Num> {
    let mut nums: Vec<Num> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    nums.sort_unstable();
    nums.insert(0, 0);
    nums
}

fn part1(nums: &[Num]) {
    let mut ones = 0;
    let mut threes = 1;
    for w in nums.windows(2) {
        if w[1] == w[0] + 3 {
            threes += 1;
        } else if w[1] == w[0] + 1 {
            ones += 1;
        }
    }
    println!(
        "PART 1: diff ones[{}] threes[{}] {}",
        ones,
        threes,
        ones * threes
    );
}

use std::collections::HashMap;
fn build_graph(nums: &[Num]) -> HashMap<Num, Vec<Num>> {
    let mut graph: HashMap<Num, Vec<Num>> = HashMap::default();

    for last_index in 0..nums.len() {
        // we need to check what numbers can go after this one to expand them
        let acc_diff = nums[last_index];
        let mut index_to_add = last_index + 1;
        loop {
            if let Some(n) = nums.get(index_to_add) {
                let diff = n - nums[last_index];
                if diff <= 3 {
                    graph.entry(acc_diff).or_insert(vec![]).push(*n);
                    index_to_add += 1;
                } else {
                    // anything beyond here does not fit the rules
                    break;
                }
            } else {
                break;
            }
        }
    }

    graph
}

fn count_paths(nums: &[Num], graph: &HashMap<Num, Vec<Num>>) -> usize {
    let target = *nums.last().unwrap();
    let mut known_counts: HashMap<Num, usize> = HashMap::new();
    for n in nums.iter().rev() {
        if *n == target {
            known_counts.insert(*n, 1);
            continue;
        }

        let count = graph
            .get(n)
            .unwrap()
            .iter()
            .map(|k| known_counts.get(k).unwrap())
            .sum();

        known_counts.insert(*n, count);
    }

    *known_counts.get(&0).unwrap()
}

fn part2(nums: &[Num]) {
    let graph = build_graph(&nums);
    let count = count_paths(&nums, &graph);
    println!("working_ones {}", count);
}

fn main() {
    let data = "data/chris.txt";
    let nums = load_sorted(data);
    part1(&nums);
    part2(&nums);
}
