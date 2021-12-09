fn main() {
    let mut input = parse_input::<9>("4,2,4,1,5,1,2,2,4,1,1,2,2,2,4,4,1,2,1,1,4,1,2,1,2,2,2,2,5,2,2,3,1,4,4,4,1,2,3,4,4,5,4,3,5,1,2,5,1,1,5,5,1,4,4,5,1,3,1,4,5,5,5,4,1,2,3,4,2,1,2,1,2,2,1,5,5,1,1,1,1,5,2,2,2,4,2,4,2,4,2,1,2,1,2,4,2,4,1,3,5,5,2,4,4,2,2,2,2,3,3,2,1,1,1,1,4,3,2,5,4,3,5,3,1,5,5,2,4,1,1,2,1,3,5,1,5,3,1,3,1,4,5,1,1,3,2,1,1,1,5,2,1,2,4,2,3,3,2,3,5,1,5,1,2,1,5,2,4,1,2,4,4,1,5,1,1,5,2,2,5,5,3,1,2,2,1,1,4,1,5,4,5,5,2,2,1,1,2,5,4,3,2,2,5,4,2,5,4,4,2,3,1,1,1,5,5,4,5,3,2,5,3,4,5,1,4,1,1,3,4,4,1,1,5,1,4,1,2,1,4,1,1,3,1,5,2,5,1,5,2,5,2,5,4,1,1,4,4,2,3,1,5,2,5,1,5,2,1,1,1,2,1,1,1,4,4,5,4,4,1,4,2,2,2,5,3,2,4,4,5,5,1,1,1,1,3,1,2,1");
    for _ in 0..80 {
        evolve(&mut input);
    }

    println!("After 80 days (part 1): {}", input.iter().sum::<usize>());

    for _ in 80..256 {
        evolve(&mut input)
    }
    println!("After 256 days (part 2): {}", input.iter().sum::<usize>());
}

fn parse_input<const N: usize>(input: &str) -> [usize; N] {
    let mut counts = [0; N];
    let input = input
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()
        .expect("Input is ok");

    for days_left in input {
        counts[days_left] += 1;
    }

    counts
}

fn evolve<const N: usize>(counts: &mut [usize; N]) {
    for (days_left, count) in counts.clone().into_iter().enumerate() {
        if count == 0 {
            continue;
        }

        if days_left == 0 {
            counts[8] += count;
            counts[6] += count;
        } else {
            counts[days_left - 1] += count;
        }

        counts[days_left] -= count;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn evolve_works() {
        let mut expected_states = "
            2,3,2,0,1
            1,2,1,6,0,8
            0,1,0,5,6,7,8
            6,0,6,4,5,6,7,8,8
            5,6,5,3,4,5,6,7,7,8
            4,5,4,2,3,4,5,6,6,7
            3,4,3,1,2,3,4,5,5,6
            2,3,2,0,1,2,3,4,4,5
            1,2,1,6,0,1,2,3,3,4,8
            0,1,0,5,6,0,1,2,2,3,7,8
            6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
            5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
            4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
            3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
            2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
            1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
            0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
            6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
        "
        .trim()
        .lines()
        .map(parse_input::<9>)
        .enumerate();

        let mut input = parse_input::<9>("3,4,3,1,2");
        while let Some((day, expected_state)) = expected_states.next() {
            evolve(&mut input);
            assert_eq!(input, expected_state, "day {}", day);
        }
    }

    #[test]
    fn example_1() {
        let mut input = parse_input::<9>("3,4,3,1,2");
        for _ in 0..80 {
            evolve(&mut input);
        }
        assert_eq!(input.iter().sum::<usize>(), 5934);

        for _ in 80..256 {
            evolve(&mut input)
        }
        assert_eq!(input.iter().sum::<usize>(), 26984457539);
    }
}
