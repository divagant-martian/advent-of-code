use intcode::get_data_from_path;
use intcode::program::{Int, Program};
use std::collections::HashSet;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let mut wtf = b"A,A,B,C,A,C,A,B,C,B\nR,12,L,8,R,6\nR,12,L,6,R,6,R,8,R,6\nL,8,R,8,R,6,R,12\nn\n";
    // let mut wtf = b"A,B,A,C,A,B,C,B,C,B\nL,10,R,8,L,6,R,6\nL,8,L,8,R,8\nR,8,L,6,L,10,L,10\nn\n";
    let mut input = wtf.iter().rev().map(|&c| c as Int).collect();
    let mut output = vec![];
    let mut data = get_data_from_path("data/input.txt");
    // let mut data = get_data_from_path("data/day_17");
    data[0] = 2;
    let mut prog = Program::new(&data, &mut input, &mut output);
    prog.run();
    println!(
        "{}",
        String::from_utf8(output.iter().map(|&c| c as u8).collect()).unwrap()
    );
    println!("{:?}", output.pop());
    // part1();
}
fn part1() {
    let mut input = vec![];
    let mut output = vec![];
    let data = get_data_from_path("data/input.txt");
    // let data = get_data_from_path("data/day_17");
    let mut program = Program::new(&data, &mut input, &mut output);
    program.run();
    let output: Vec<_> = output.iter().map(|&x| x as u8 as char).collect();
    //     let output: Vec<_> = "..#..........
    // ..#..........
    // #######...###
    // #.#...#...#.#
    // #############
    // ..#...#...#..
    // ..#####...^..
    // "
    //     .chars()
    //     .collect();

    let width = 1 + output
        .iter()
        .position(|&c| c == '\n')
        .expect("\\n not found");
    println!("{}/{}", output.len(), width);
    let mut blocks = HashSet::new();
    for (ym1, ch) in output.chunks(width).enumerate() {
        for (xm1, &c) in ch.iter().enumerate() {
            if c == '#' {
                blocks.insert((xm1 + 1, ym1 + 1));
            }
        }
    }
    let mut intersections = HashSet::new();
    let ans: usize = blocks
        .iter()
        .filter_map(|&(x, y)| {
            if blocks.contains(&(x, y - 1))
                && blocks.contains(&(x, y + 1))
                && blocks.contains(&(x + 1, y))
                && blocks.contains(&(x - 1, y))
            {
                intersections.insert((x, y));
                return Some((x - 1) * (y - 1));
            }
            None
        })
        .sum();
    println!("{:?}", intersections);
    println!("ans  {}", ans);
    println!("{}", output.iter().collect::<String>());
}
