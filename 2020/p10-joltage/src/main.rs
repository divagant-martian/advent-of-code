use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn load_sorted(filename: &'static str) -> Vec<usize> {
    let mut nums: Vec<usize> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    nums.sort_unstable();
    nums.insert(0, 0);
    nums
}

fn part1(nums: &[usize]) {
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

#[derive(Debug)]
struct SearchState {
    acc_diff: usize,
    used_indexes: Vec<usize>,
    last_index: usize,
}

impl SearchState {
    fn print(&self, prefix: &'static str, nums: &[usize]) {
        let mut used_nums = Vec::with_capacity(self.used_indexes.len() + 1);
        for ix in &self.used_indexes {
            used_nums.push(nums[*ix]);
        }
        used_nums.push(nums[self.last_index]);
        println!(
            "{} acc_diff[{}], used_nums:[{:?}]",
            prefix, self.acc_diff, used_nums
        );
    }
}

fn main() {
    let data = "data/test1.txt";
    let mut nums = load_sorted(data);
    let target = *nums.last().unwrap() + 3;
    dbg!(target);
    nums.push(target);

    let mut to_expand = VecDeque::new();
    to_expand.push_back(SearchState {
        acc_diff: 0,
        used_indexes: vec![],
        last_index: 0,
    });

    let mut working_sums = 0;

    while let Some(SearchState {
        acc_diff,
        used_indexes,
        last_index,
    }) = to_expand.pop_front()
    {
        // we need to check what numbers can go after this one to expand them
        let mut index_to_add = last_index + 1;
        loop {
            if let Some(n) = nums.get(index_to_add) {
                let diff = n - nums[last_index];
                if diff <= 3 {
                    // this one fits the rules, now check the sum
                    match (acc_diff + diff).cmp(&target) {
                        Ordering::Less => {
                            // this one could work, add this number to expand it
                            let mut used_indexes = used_indexes.clone();
                            used_indexes.push(last_index);
                            let state = SearchState {
                                acc_diff: acc_diff + diff,
                                used_indexes,
                                last_index: index_to_add,
                            };
                            state.print("adding ", &nums);
                            to_expand.push_back(state);
                            index_to_add += 1;
                        }
                        Ordering::Equal => {
                            // this one works. Do not expand it
                            working_sums += 1;
                            let state = SearchState {
                                acc_diff: acc_diff + diff,
                                used_indexes,
                                last_index: index_to_add,
                            };
                            state.print("WORKING ", &nums);
                            break;
                        }
                        Ordering::Greater => {
                            // does not work, do not expand more
                            // println!("diff would be too large");
                            break;
                        }
                    }
                } else {
                    // anything beyond here does not fit the rules
                    // println!("continuing won't fit the rules {}", n);
                    break;
                }
            } else {
                // println!("trying with index too large");
                break;
            }
        }
    }

    println!("working_ones {}", working_sums);
}
