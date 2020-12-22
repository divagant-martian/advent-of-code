use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::fs::read_to_string;

type Card = u16;
type Score = usize;

fn main() {
    let path = std::env::args().nth(1).expect("no path provided");
    let file_contents = read_to_string(path).expect("bad input");
    let mut content = file_contents.split("\n\n");

    let player_1 = parse_deck(content.next().unwrap());
    let player_2 = parse_deck(content.next().unwrap());

    println!("Part 1: {:?}", combat(player_1.clone(), player_2.clone()));
    println!(
        "Part 2: {:?}",
        recursive_combat(player_1.clone(), player_2.clone())
    );
}

fn score(deck: &VecDeque<Card>) -> Score {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, card)| *card as Score * (idx + 1))
        .sum::<Score>()
}

fn combat(
    mut player_1: VecDeque<Card>,
    mut player_2: VecDeque<Card>,
) -> (&'static str /* Winner */, Score /* Score */) {
    while !player_1.is_empty() && !player_2.is_empty() {
        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(card_1), Some(card_2)) => {
                if card_1 > card_2 {
                    player_1.push_back(card_1);
                    player_1.push_back(card_2);
                } else {
                    player_2.push_back(card_2);
                    player_2.push_back(card_1);
                }
            }
            _ => unreachable!("Both players have cards"),
        }
    }

    if player_1.is_empty() {
        ("Player 2", score(&player_2))
    } else {
        ("Player 1", score(&player_1))
    }
}

fn recursive_combat(player_1: VecDeque<Card>, player_2: VecDeque<Card>) -> (&'static str, Score) {
    match recursive_combat_rec(player_1, player_2) {
        Ok(score) => ("Player 1", score),
        Err(score) => ("Player 2", score),
    }
}

fn recursive_combat_rec(
    mut player_1: VecDeque<Card>,
    mut player_2: VecDeque<Card>,
) -> Result<Score, Score> /* Ok is 1 wins, Err if 2 wins */
{
    let mut seen_scores = BTreeSet::new();

    while !player_1.is_empty() && !player_2.is_empty() {
        let (p1_score, p2_score) = (score(&player_1), score(&player_2));
        if !seen_scores.insert((p1_score, p2_score)) {
            break;
        }
        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(card_1), Some(card_2)) => {
                if player_1.len() >= card_1 as usize && player_2.len() >= card_2 as usize {
                    let new_player_1 = player_1.iter().take(card_1 as usize).cloned().collect();
                    let new_player_2 = player_2.iter().take(card_2 as usize).cloned().collect();
                    if recursive_combat_rec(new_player_1, new_player_2).is_ok() {
                        // player 1 won by recursive combat
                        player_1.push_back(card_1);
                        player_1.push_back(card_2);
                    } else {
                        // player 2 won by recursive combat
                        player_2.push_back(card_2);
                        player_2.push_back(card_1);
                    }
                } else if card_1 > card_2 {
                    // player 1 won by high value
                    player_1.push_back(card_1);
                    player_1.push_back(card_2);
                } else {
                    // player 2 won by high value
                    player_2.push_back(card_2);
                    player_2.push_back(card_1);
                }
            }
            _ => unreachable!("Both players have cards"),
        }
    }

    if !player_1.is_empty() {
        Ok(score(&player_1))
    } else {
        Err(score(&player_2))
    }
}

fn parse_deck(repr: &str) -> VecDeque<Card> {
    repr.lines().skip(1).map(|n| n.parse().unwrap()).collect()
}
