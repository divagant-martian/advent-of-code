#[derive(Debug)]
enum Ins {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn parse_instruction(line: &str) -> Ins {
    let mut parts = line.split_ascii_whitespace();
    let direction = match parts.next().expect("Expected instruction") {
        "forward" => Ins::Forward,
        "up" => Ins::Up,
        "down" => Ins::Down,
        other => panic!("Unkown instruction {}", other),
    };
    let magnitude =
        str::parse(parts.next().expect("Instruction needs a magnitude")).expect("Wrong magnitude");
    direction(magnitude)
}

fn final_depth_linear_mov(init_pos: (usize, usize), movements: &[Ins]) -> (usize, usize) {
    movements
        .into_iter()
        .fold(init_pos, |(x, y), ins| match ins {
            Ins::Forward(n) => (x + n, y),
            Ins::Up(n) => (x, y - n),
            Ins::Down(n) => (x, y + n),
        })
}

fn final_depth_weird_move(
    init_pos: (usize, usize, usize),
    movements: &[Ins],
) -> (usize, usize, usize) {
    movements
        .into_iter()
        .fold(init_pos, |(x, y, aim), ins| match ins {
            Ins::Forward(n) => (x + n, y + aim * n, aim),
            Ins::Up(n) => (x, y, aim - n),
            Ins::Down(n) => (x, y, aim + n),
        })
}

fn main() {
    let instructions = std::fs::read_to_string("data/input.txt")
        .expect("File exists")
        .lines()
        .map(parse_instruction)
        .collect::<Vec<_>>();

    let final_pos = final_depth_linear_mov((0, 0), &instructions);
    println!("{:?}", final_pos);
    println!("{:?}", final_pos.0 * final_pos.1);

    let final_pos = final_depth_weird_move((0, 0, 0), &instructions);
    println!("2: {:?}", final_pos);
    println!("2: {:?}", final_pos.0 * final_pos.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let instructions = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .into_iter()
        .map(parse_instruction)
        .collect::<Vec<_>>();

        let final_pos = final_depth_linear_mov((0, 0), &instructions);
        assert_eq!(final_pos, (15, 10));

        let (x, y, _aim) = final_depth_weird_move((0, 0, 0), &instructions);
        assert_eq!((x, y), (15, 60));
    }
}
