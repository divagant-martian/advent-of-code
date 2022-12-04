use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Item(char);

impl Item {
    pub fn priority(&self) -> u8 {
        let adjustment: u8 = self.0.is_uppercase().then_some(58).unwrap_or(0);
        self.0 as u8 + adjustment - 96
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Needs a file");
    let data = std::fs::read_to_string(file_name).expect("File exists");
    let items: Vec<_> = data
        .lines()
        .map(|l| {
            let mut rucksack = l.chars().map(Item).collect::<Vec<_>>();
            let size = rucksack.len();
            let second_compartment = rucksack.split_off(size / 2);
            (rucksack, second_compartment)
        })
        .collect();
    let misplaced: usize = items
        .into_iter()
        .map(|(first, second)| {
            let first: HashSet<Item> = HashSet::from_iter(first.into_iter());
            let second = HashSet::from_iter(second.into_iter());
            first
                .intersection(&second)
                .into_iter()
                .next()
                .unwrap()
                .priority() as usize
        })
        .sum();
    println!("{:?}", misplaced);
}
