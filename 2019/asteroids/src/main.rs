use asteroids::{solve_ten_a, solve_ten_b};
use std::env;

fn main() {
    let mut args = env::args();
    let path: String = args.nth(1).expect("no data path provided");

    let (as0, dist) = solve_ten_a(&path).expect("10a shat the bed");
    println!(
        "10) Part A: asteroid {:?} can detect {:?} asteroids",
        as0, dist
    );
    let nth = args
        .next()
        .expect("give me the nth to find and destroy")
        .trim()
        .parse()
        .expect("the nth is not a good number");
    let ans = solve_ten_b(&path, as0, nth);
    println!("    Part B: the {}th vaporized asteroid is {:?}", nth, ans);
}
