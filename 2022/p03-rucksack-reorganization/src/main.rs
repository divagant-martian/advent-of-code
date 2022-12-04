use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Item(char);

impl Item {
    #[track_caller]
    pub fn priority(&self) -> u8 {
        let adjustment: u8 = self.0.is_uppercase().then_some(58).unwrap_or(0);
        self.0 as u8 + adjustment - 96
    }
}

fn main() {
    problem_2()
}

fn problem_1() {
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

fn problem_2() {
    let file_name = std::env::args().nth(1).expect("Needs a file");
    let data = std::fs::read_to_string(file_name).expect("File exists");
    let items = data
        .lines()
        .map(|l| l.chars().map(Item).collect::<HashSet<_>>());
    let mut advanced = 0;
    let mut accum_priority = 0usize;
    let mut seen_items_in_group = HashSet::default();
    for rucksack in items {
        if seen_items_in_group.is_empty() {
            seen_items_in_group = rucksack.clone();
        } else {
            seen_items_in_group = seen_items_in_group
                .intersection(&rucksack)
                .cloned()
                .collect();
        }
        advanced += 1;
        if advanced % 3 == 0 {
            assert_eq!(seen_items_in_group.len(), 1);
            let common_item = seen_items_in_group
                .iter()
                .next()
                .expect("There must be a common badge");
            accum_priority += common_item.priority() as usize;
            seen_items_in_group.clear();
        }
    }
    println!("{accum_priority}");
}
