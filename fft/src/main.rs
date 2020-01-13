use rayon::prelude::*;

fn get_data(raw: &str) -> (Vec<i32>, usize) {
    (
        raw.trim()
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid digit in file") as i32)
            .cycle()
            .take(raw.len() * 10_000)
            .collect(),
        raw[0..7].parse::<usize>().unwrap(),
    )
}

fn main() {
    let input = "59790132880344516900093091154955597199863490073342910249565395038806135885706290664499164028251508292041959926849162473699550018653393834944216172810195882161876866188294352485183178740261279280213486011018791012560046012995409807741782162189252951939029564062935408459914894373210511494699108265315264830173403743547300700976944780004513514866386570658448247527151658945604790687693036691590606045331434271899594734825392560698221510565391059565109571638751133487824774572142934078485772422422132834305704887084146829228294925039109858598295988853017494057928948890390543290199918610303090142501490713145935617325806587528883833726972378426243439037";
    let init_phase = 0;
    let target_pase = 100;

    let (mut digits, offset) = get_data(&input);
    let size = digits.len();
    for p in init_phase..target_pase {
        let now = std::time::Instant::now();
        digits = (1..=size)
            .into_par_iter()
            .map(|i| {
                if i < offset {
                    return 0;
                }

                let pos_bot = i - 1;
                let pos_top = 2 * i - 1;

                let neg_bot = 3 * i - 1;
                let neg_top = 4 * i - 1;

                let period = 4 * i;

                let mut tot = 0;

                let mut bot = pos_bot;
                let mut top = pos_top;
                while bot < size {
                    tot += digits[bot..top.min(size)].iter().sum::<i32>().abs() % 10;
                    bot += period;
                    top += period;
                }

                bot = neg_bot;
                top = neg_top;
                while bot < size {
                    tot -= digits[bot..top.min(size)].iter().sum::<i32>().abs() % 10;
                    bot += period;
                    top += period;
                }

                tot.abs() % 10
            })
            .collect();

        println!("Phase {} lasted {} millisecs", p, now.elapsed().as_millis());
    }
    println!(
        "{}",
        digits[offset..offset + 8]
            .into_iter()
            .map(|n| n.to_string())
            .collect::<String>()
    );
}
