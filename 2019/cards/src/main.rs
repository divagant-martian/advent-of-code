mod deck;
mod shuffle_trick;

use crate::deck::{IndexSim, Shufflable, Shuffle};
use crate::shuffle_trick::*;

fn part1() {
    let nc = 10007;
    // let nc = 10;
    let mut deck = (0..nc).collect::<Vec<usize>>();
    let tricks = parse_tricks("data/input.txt");
    for trick in tricks.iter() {
        deck.shuffle(trick);
    }

    for (i, n) in deck.iter().enumerate() {
        if *n == 2019 {
            println!("Part 1 {}", i);
            break;
        }
    }
}

// -----------------------------PART 2-------------------------------

fn part2() {
    // let nc = 10007;
    let nexec = 1;
    let nc = 119315717514047_usize;
    // let nexec = 101741582076661_usize;

    let tricks = parse_inverse_tricks("data/input.txt", nc);

    let mut find = IndexSim::new(2020, nc);
    for e in 0..nexec {
        for trick in &tricks {
            find.shuffle(trick);
        }
    }

    println!("found {:?}", find);
}

fn main() {
    // part2();
    // let mut deck = (0..10).collect::<Vec<usize>>();
    // let tricks = [DealInc(9), DealInc(3), DealInc(7), DealInc(9)];
    // for trick in tricks.iter() {
    //     deck.shuffle(trick);
    //     println!("{:?}", deck);
    // }
    //
    let path = "data/input.txt";
    // let path = "data/test.txt";

    let (i, nc) = (2019, 10007);
    let (i, nc) = (2020, 119315717514047_usize);
    // let (i, nc) = (1, 10);
    let mut todo = IndexSim::new(i, nc);
    println!("Started with {:?}", todo); // what we get

    // do one direction
    let tricks = parse_tricks(path);
    for trick in tricks {
        todo.shuffle(&trick);
    }
    println!("Forward {:?}", todo); // what we get
                                    // do reversal
    let tricks = parse_inverse_tricks(path, nc);
    for trick in tricks {
        todo.shuffle(&trick);
    }
    println!("Backwards {:?}", todo); // what we get
}
