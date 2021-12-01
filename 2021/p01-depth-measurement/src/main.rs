fn read_input(contents: String) -> Vec<usize> {
    contents
        .lines()
        .map(|line| str::parse::<usize>(line).expect("Bad number"))
        .collect::<Vec<_>>()
}

fn num_increased(nums: &[usize]) -> usize {
    nums.windows(2).filter(|slice| slice[1] > slice[0]).count()
}

fn main() {
    let contents = std::fs::read_to_string("data/input1.txt").expect("File is ok");
    let nums = read_input(contents);
    println!("ans: {}", num_increased(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, num_increased(&nums));
    }
}
