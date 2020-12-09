use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn read_xmax(filename: &'static str) -> Vec<usize> {
    read_to_string(filename)
        .expect("bad input")
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn is_sum(window: &[usize], preamble: usize) -> bool {
    for i in 0..preamble {
        for j in i..preamble {
            if window[i] + window[j] == window[preamble] {
                return true;
            }
        }
    }
    false
}

fn get_failing_target(nums: &[usize], preamble: usize) -> Option<usize> {
    for window in nums.windows(preamble + 1) {
        if !is_sum(window, preamble) {
            return Some(window[preamble]);
        }
    }
    None
}

fn main() {
    let preamble = 25;
    let nums = read_xmax("data/input1.txt");
    let target = get_failing_target(&nums, preamble).unwrap();

    let mut added_nums = VecDeque::new();
    let mut acc_sum = 0;
    for n in nums {
        match (acc_sum + n).cmp(&target) {
            Ordering::Equal => {
                println!(
                    "smallest and greatest are {:?} and {:?}",
                    added_nums.front(),
                    added_nums.back()
                );
                break;
            }
            Ordering::Less => {
                added_nums.push_back(n);
                acc_sum += n;
            }
            Ordering::Greater => {
                acc_sum += n;
                added_nums.push_back(n);
                while let Some(first) = added_nums.pop_front() {
                    acc_sum -= first;
                    if acc_sum + first <= target {
                        added_nums.push_front(first);
                        acc_sum += first;
                        break;
                    }
                }
                if acc_sum == target {
                    let max = added_nums.iter().max();
                    let min = added_nums.iter().min();
                    println!(
                        "smallest and greatest, after Greater are {:?} and {:?}, ans: {}",
                        min,
                        max,
                        min.unwrap() + max.unwrap()
                    );
                    break;
                }
            }
        }
    }
}
