mod part_1;

use part_1::Board;
use std::collections::HashMap;

const OFFSET: usize = 1;
const SIZE: usize = 5;

type RecursiveBoard = HashMap<isize, Board>;

fn positions_to_check_other_levels(
    pos: &(usize, usize),
    level: isize,
) -> Vec<(isize, (usize, usize))> {
    let inner_up = (3, 2);
    let inner_left = (2, 3);
    let inner_right = (4, 3);
    let inner_down = (3, 4);
    let outer_down = vec![(1, 5), (2, 5), (3, 5), (4, 5), (5, 5)];
    let outer_up = vec![(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)];
    let outer_left = vec![(1, 1), (1, 2), (1, 3), (1, 4), (1, 5)];
    let outer_right = vec![(5, 1), (5, 2), (5, 3), (5, 4), (5, 5)];
    let mut check = vec![];
    match pos {
        (1, 1) => {
            check.push((level - 1, inner_up));
            check.push((level - 1, inner_left));
        }
        (5, 1) => {
            check.push((level - 1, inner_up));
            check.push((level - 1, inner_right));
        }
        (1, 5) => {
            check.push((level - 1, inner_left));
            check.push((level - 1, inner_down));
        }
        (5, 5) => {
            check.push((level - 1, inner_right));
            check.push((level - 1, inner_down));
        }
        pos if outer_left.contains(pos) => check.push((level - 1, inner_left)),
        pos if outer_right.contains(pos) => check.push((level - 1, inner_right)),
        pos if outer_up.contains(pos) => check.push((level - 1, inner_up)),
        pos if outer_down.contains(pos) => check.push((level - 1, inner_down)),
        pos if pos == &inner_up => {
            for o in outer_up {
                check.push((level + 1, o));
            }
        }
        pos if pos == &inner_right => {
            for o in outer_right {
                check.push((level + 1, o));
            }
        }
        pos if pos == &inner_left => {
            for o in outer_left {
                check.push((level + 1, o));
            }
        }
        pos if pos == &inner_down => {
            for o in outer_down {
                check.push((level + 1, o));
            }
        }
        _ => (),
    }
    check
}

fn near_bugs_rec(board: &RecursiveBoard, pos: &(usize, usize), level: isize) -> usize {
    let mut bugs = 0;
    if let Some(current_lvl) = board.get(&level) {
        bugs += part_1::near_bugs(&current_lvl, pos);
    }
    let check = positions_to_check_other_levels(pos, level);
    for (l, p) in check {
        if let Some(other_lvl) = board.get(&l) {
            if *other_lvl.get(&p).unwrap() {
                bugs += 1;
            }
        }
    }
    bugs
}

fn evolve_rec(board: &RecursiveBoard) -> RecursiveBoard {
    let (upper, lower) = board
        .keys()
        .into_iter()
        .fold((0, 0), |(max, min), &l| (max.max(l), min.min(l)));
    // fake two more levels: one up other down
    let mut board = board.clone();
    let mut up = HashMap::new();
    let mut down = HashMap::new();
    for x in 1..=5 {
        for y in 1..=5 {
            if (x, y) != (3, 3) {
                up.insert((x, y), false);
                down.insert((x, y), false);
            }
        }
    }
    board.insert(upper + 1, up);
    board.insert(lower - 1, down);
    let mut new = HashMap::new();
    for (&level, level_board) in &board {
        let mut new_level = HashMap::new();
        for (&pos, &infested) in level_board {
            let n_bugs = near_bugs_rec(&board, &pos, level);
            if infested && n_bugs != 1 {
                new_level.insert(pos, false);
            } else if !infested && (n_bugs == 1 || n_bugs == 2) {
                new_level.insert(pos, true);
            } else {
                new_level.insert(pos, infested);
            }
        }
        new.insert(level, new_level);
    }

    new
    // unimplemented!()
}

fn parse_board_rec(rep: &str) -> RecursiveBoard {
    let mut lvl0 = part_1::parse_board(rep);
    let mut board = HashMap::new();
    lvl0.remove(&(3, 3)).unwrap();
    board.insert(0, lvl0);
    board
}

fn paint_board_rec(board: &RecursiveBoard) {
    let mut levels = board.keys().collect::<Vec<_>>();
    levels.sort();
    for l in levels {
        println!("\nLEVEL {}", l);
        part_1::paint_board(board.get(l).unwrap());
    }
}

fn part2() {
    let rep = "\
#..##
#.#..
#...#
##..#
#..##\
";
    let mut board = parse_board_rec(rep);
    paint_board_rec(&board);
    println!("evolved----------------------");
    for _ in 0..200 {
        board = evolve_rec(&board);
    }
    // paint_board_rec(&board);
    let nbugs = board
        .into_iter()
        .map(|(_, b)| b.into_iter().map(|(_, v)| v as usize).sum::<usize>())
        .sum::<usize>();
    println!("found {} bugs", nbugs);
}

fn main() {
    println!("{:-<80}", "- PART 1 -");
    part_1::part1();
    println!("{:-<80}", "- PART 2 -");
    part2();
}
