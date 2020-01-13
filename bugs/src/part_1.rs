use crate::{OFFSET, SIZE};
use std::collections::{HashMap, HashSet};

pub type Board = HashMap<(usize, usize), bool>;

pub fn near_bugs(board: &Board, pos: &(usize, usize)) -> usize {
    let &(x, y) = pos;
    let mut n_bugs = 0;
    for n in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        n_bugs += board.get(n).cloned().unwrap_or_default() as usize;
    }
    n_bugs
}

fn evolve(board: &Board) -> Board {
    let mut new = HashMap::with_capacity(board.len());
    for (&pos, &infested) in board {
        let n_bugs = near_bugs(board, &pos);
        if infested && n_bugs != 1 {
            new.insert(pos, false);
        } else if !infested && (n_bugs == 1 || n_bugs == 2) {
            new.insert(pos, true);
        } else {
            new.insert(pos, infested);
        }
    }
    new
}

pub fn parse_board(rep: &str) -> Board {
    let mut board = HashMap::new();
    for (y, l) in rep.lines().enumerate() {
        for (x, c) in l.char_indices() {
            if c == '#' {
                board.insert((x + OFFSET, y + OFFSET), true);
            } else {
                board.insert((x + OFFSET, y + OFFSET), false);
            }
        }
    }
    board
}

fn biodiveristy(board: &Board) -> u32 {
    let mut rating = 0;
    for ((x, y), &infested) in board {
        if infested {
            rating += 2_u32.pow(((x - OFFSET) + SIZE * (y - OFFSET)) as u32);
        }
    }
    rating
}

fn evolve_until_repeat(board: &Board) -> (Board, u32) {
    let mut board = board.clone();
    let mut rating = biodiveristy(&board);

    let mut seen = HashSet::new();
    seen.insert(rating);
    while {
        board = evolve(&board);
        rating = biodiveristy(&board);
        seen.insert(rating)
    } {}
    (board.clone(), rating)
}

pub fn paint_board(board: &Board) {
    let max = OFFSET + SIZE;
    for y in OFFSET..max {
        let mut row = String::with_capacity(SIZE);
        for x in OFFSET..max {
            if let Some(infested) = board.get(&(x, y)).cloned() {
                if infested {
                    row.push('#');
                } else {
                    row.push('.');
                }
            } else {
                row.push('?');
            }
        }
        println!("{}", row);
    }
}

pub fn part1() {
    let rep = "\
#..##
#.#..
#...#
##..#
#..##\
";
    let board = parse_board(rep);
    paint_board(&board);
    println!("rating: {}", biodiveristy(&board));
    let (repeated, rating) = evolve_until_repeat(&board);
    paint_board(&repeated);
    println!("rating: {}", rating);
}
