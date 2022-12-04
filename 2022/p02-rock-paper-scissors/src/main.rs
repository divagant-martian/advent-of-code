use crate::strategy::Strategy;

mod round;
mod shape;
mod strategy;

fn main() {
    let file_name = std::env::args().nth(1).expect("give a file name to use");
    let data = std::fs::read_to_string(file_name).expect("File exists");
    let strategy: Strategy = data.parse().expect("Data is right");
    let score: usize = strategy.execute().map(|x| x.score()).sum();
    println!("{score}");
}
