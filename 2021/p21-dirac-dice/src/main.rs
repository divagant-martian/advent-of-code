fn main() {
    println!("Hello, world!");
    // for _ in 0..444356092776315usize {
    play_game(10, 7)
    // }
}

pub fn play_game(mut player1: usize, mut player2: usize) {
    let mut dice = (1..=100).cycle();
    let mut p1_score = 0usize;
    let mut p2_score = 0usize;

    let mut times = 0;
    let mut last_is_p1 = false;
    'a: loop {
        for (p, player, score) in [
            (1, &mut player1, &mut p1_score),
            (2, &mut player2, &mut p2_score),
        ] {
            last_is_p1 = !last_is_p1;
            for _ in 0..3 {
                *player += dice.next().unwrap();
                times += 1;
                *player = (*player - 1) % 10 + 1;
            }
            *score += *player;
            dbg!((p, player, *score));
            if *score >= 1000 {
                break 'a;
            }
        }
    }
    if last_is_p1 {
        dbg!(times, p2_score, times * p2_score)
    } else {
        dbg!(times, p1_score, times * p1_score)
    };
}
