use asteroids::solve;
use std::env;

fn main() {
    let mut args = env::args();
    let path: String = args.nth(1).expect("no data path provided");

    println!("{:?}", solve(&path));
}
