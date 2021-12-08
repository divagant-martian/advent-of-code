use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./data/input.txt").expect("Input is ok");
    let (draws, mut boards) = parse_input(&input);

    let wins = play(&draws, &mut boards);

    let (winning_draw_position, winning_board_position) = wins[0];
    let score = calculate_score(
        draws[winning_draw_position],
        &boards[winning_board_position],
    );
    println!("Score: {}", score);

    let &(last_winning_draw_position, last_winning_board_position) =
        wins.last().expect("More than a board will win");
    let score = calculate_score(
        draws[last_winning_draw_position],
        &boards[last_winning_board_position],
    );
    println!("last score: {}", score);
}

type Board = Vec<Vec<(u8, bool)>>;

fn parse_board(input: &str) -> Board {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<u8>().expect("Numbers are ok"))
                .map(|num| (num, false))
                .collect()
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut parts = input.split("\n\n");
    let draws = parts
        .next()
        .expect("Draws must be present")
        .trim()
        .split(',')
        .map(|num_str| num_str.parse::<u8>().expect("input number is ok"))
        .collect();

    let boards = parts.map(parse_board).collect();
    (draws, boards)
}

fn mark_number(n: u8, board: &mut Board) -> bool {
    let mut maybe_position = None;
    for (row_n, row) in board.iter_mut().enumerate() {
        for (col_n, (num, was_marked)) in row.iter_mut().enumerate() {
            if *num == n {
                *was_marked = true;
                maybe_position = Some((row_n, col_n));
                break;
            }
        }
    }

    if let Some((row, col)) = maybe_position {
        let row_complete = board[row].iter().all(|(_n, was_marked)| *was_marked);
        if row_complete {
            return true;
        }

        let column_complete = board.iter().all(|row| row[col].1);
        if column_complete {
            return true;
        }
    }

    false
}

fn format_board(board: &Board) -> String {
    board
        .iter()
        .map(|row| row.iter().map(|(n, _was_marked)| n).collect::<Vec<_>>())
        .map(|row| format!("{:?}\n", row))
        .collect()
}

fn play(
    draws: &Vec<u8>,
    boards: &mut Vec<Board>,
) -> Vec<(
    usize, /* winning draw position */
    usize, /* winning board position */
)> {
    let mut wins = Vec::new();
    let mut already_won_boards: HashSet<usize> = HashSet::default();
    for (i, n) in draws.iter().enumerate() {
        for (board_number, board) in boards.iter_mut().enumerate() {
            if !already_won_boards.contains(&board_number) && mark_number(*n, board) {
                wins.push((i, board_number));
                already_won_boards.insert(board_number);
            }
        }
    }
    wins
}

fn calculate_score(winning_n: u8, winning_board: &Board) -> u32 {
    let non_marked_sum: u32 = winning_board
        .iter()
        .map(|row| {
            row.iter()
                .filter_map(|(n, was_marked)| (!was_marked).then(|| *n as u32))
                .sum::<u32>()
        })
        .sum();
    non_marked_sum * (winning_n as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mark_bingo() {
        let board = "
            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        ";

        let mut board = parse_board(board);
        let draws = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21];
        let winning_n = 24;

        for n in draws {
            assert!(!mark_number(n, &mut board));
        }
        assert!(mark_number(winning_n, &mut board));
    }

    #[test]
    fn example() {
        let input = "
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        ";

        let (draws, mut boards) = parse_input(input);
        let mut wins = play(&draws, &mut boards);
        println!("{:?}", wins);
        let (winning_draw_position, winning_board_position) = wins.remove(0);
        let score = calculate_score(
            draws[winning_draw_position],
            &boards[winning_board_position],
        );
        assert_eq!(score, 4512);

        let &(last_winning_draw_position, last_winning_board_position) =
            wins.last().expect("More than a board will win");
        let score = calculate_score(
            draws[last_winning_draw_position],
            &boards[last_winning_board_position],
        );
        assert_eq!(score, 1924);
    }
}
