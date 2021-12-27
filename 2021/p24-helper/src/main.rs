use parking_lot::Mutex;
use std::{collections::VecDeque, sync::Arc};

use rayon::prelude::*;

// const CONSTANTS [> MINE <]: [[isize; 3]; 14] = [
//     [1, 13, 14],
//     [1, 12, 8],
//     [1, 11, 5],
//     [26, 0, 4],
//     [1, 15, 10],
//     [26, -13, 13],
//     [1, 10, 16],
//     [26, -9, 5],
//     [1, 11, 6],
//     [1, 13, 13],
//     [26, -14, 6],
//     [26, -3, 7],
//     [26, -2, 13],
//     [26, -14, 3],
// ];

const CONSTANTS /* CHRIS */: [[isize; 3]; 14] = [
    [1, 13, 6],
    [1, 15, 7],
    [1, 15, 10],
    [1, 11, 2],
    [26, -7, 15],
    [1, 10, 8],
    [1, 10, 1],
    [26, -5, 10],
    [1, 15, 5],
    [26, -3, 3],
    [26, 0, 5],
    [26, -5, 11],
    [26, -9, 12],
    [26, 0, 10],
];

fn main() {
    let mut results = expand();
    results.sort();
    println!("{:?}", results);
}

#[derive(Default)]
struct Queued {
    searched_z: isize,
    known_ws: Vec<isize>,
}

fn expand() -> Vec<[isize; 14]> {
    let queue = Arc::new(Mutex::new(VecDeque::from([Queued::default()])));
    let results = Arc::new(Mutex::new(Vec::<[isize; 14]>::new()));
    while let Some(Queued {
        searched_z,
        known_ws,
    }) = {
        let mut handle = queue.lock();
        let queued = handle.pop_front();
        drop(handle);
        queued
    } {
        let [a, b, c] = CONSTANTS[13 - known_ws.len()];
        (0..=10000000).into_par_iter().for_each(|z| {
            let range = if known_ws.len() == 13 { 1..=7 } else { 1..=9 };
            for w in range {
                if program(a, b, c, z, w) == searched_z {
                    let mut to_expand = known_ws.clone();
                    to_expand.push(w);
                    if to_expand.len() == 14 {
                        to_expand.reverse();
                        println!(
                            "[RESULT] z:{} {}",
                            z,
                            to_expand
                                .iter()
                                .map(|d| format!("{}", d.abs() as usize))
                                .collect::<String>()
                        );
                        let mut handle = results.lock();
                        handle.push(to_expand.try_into().expect("len is 14"));
                        drop(handle);
                        panic!("found");
                    } else {
                        // println!("[EXPAND] z:{} {:?}", z, to_expand);
                        let mut handle = queue.lock();
                        handle.push_front(Queued {
                            searched_z: z,
                            known_ws: to_expand,
                        });
                        drop(handle);
                    }
                }
            }
        });
    }
    Arc::try_unwrap(results).unwrap().into_inner()
}

fn program(a: isize, b: isize, c: isize, mut z: isize, w: isize) -> isize {
    let mut x = 0;
    let mut y = 0;
    x *= 0;
    x += z;
    x %= 26;
    z /= a;
    x += b;
    x = if x == w { 1 } else { 0 };
    x = if x == 0 { 1 } else { 0 };
    y *= 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += c;
    y *= x;
    z += y;

    z
}
