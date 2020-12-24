use op::{EisensteinInt, Op, OPS};
mod op;

fn parse(input: &str) -> Vec<Vec<Op>> {
    input
        .lines()
        .map(|l| {
            let mut was_north = None;
            let mut ops = Vec::with_capacity(l.len() / 2);
            for c in l.chars() {
                let push = match (was_north, c) {
                    (None, 'e') => Ok(Op::E),
                    (None, 'w') => Ok(Op::W),
                    (Some(true), 'e') => Ok(Op::NE),
                    (Some(true), 'w') => Ok(Op::NW),
                    (Some(false), 'e') => Ok(Op::SE),
                    (Some(false), 'w') => Ok(Op::SW),
                    (_, 's') => Err(false),
                    (_, 'n') => Err(true),
                    other => unreachable!("got {:?}", other),
                };
                match push {
                    Ok(op) => {
                        ops.push(op);
                        was_north = None
                    }
                    Err(new_was_north) => was_north = Some(new_was_north),
                }
            }
            ops
        })
        .collect()
}

fn main() {
    let path = std::env::args().nth(1).expect("No file given");
    let contents = parse(&std::fs::read_to_string(path).expect("Bad input file"));
    let tiles: Vec<EisensteinInt> = contents
        .iter()
        .map(|l| l.iter().fold(EisensteinInt::new(0, 0), |acc, op| &acc + op))
        .collect();

    let mut black_tiles = std::collections::HashSet::new();
    for eint in tiles {
        if black_tiles.contains(&eint) {
            black_tiles.remove(&eint);
        } else {
            black_tiles.insert(eint);
        }
    }
    dbg!(black_tiles.len());

    // Part 2
    let mut old_state: std::collections::HashSet<EisensteinInt>;
    let mut whites_to_check = Vec::new();

    for _ in 0..100 {
        old_state = black_tiles.drain().collect();
        // println!("{:?}", old_state);
        whites_to_check.clear();
        for tile in &old_state {
            let mut black_neighbors = 0;
            for op in &OPS {
                let neighbor = tile + op;
                if old_state.contains(&neighbor) {
                    black_neighbors += 1;
                } else {
                    whites_to_check.push(neighbor);
                }
            }
            if !(black_neighbors == 0 || black_neighbors > 2) {
                black_tiles.insert(tile.clone());
            }
        }

        for tile in &whites_to_check {
            let mut black_neighbors = 0;
            for op in &OPS {
                let neighbor = tile + op;
                if old_state.contains(&neighbor) {
                    black_neighbors += 1;
                }
            }
            if black_neighbors == 2 {
                black_tiles.insert(tile.clone());
            }
        }
    }

    dbg!(black_tiles.len());
}
/*
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
*/
