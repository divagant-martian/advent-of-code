use std::collections::VecDeque;

fn main() {
    let mut input = "
        7222221271
        6463754232
        3373484684
        4674461265
        1187834788
        1175316351
        8211411846
        4657828333
        5286325337
        5771324832
    "
    .parse::<Grid<10>>()
    .expect("input is ok");

    let mut count = 0;
    input.evolve_counting_flashes_n_times(&mut count, 100);
    println!("Flashes after 100: {}", count);
    println!("Sync steps: {}", 100 + input.evolve_until_synchronized());
}

#[derive(PartialEq, Eq)]
struct Grid<const N: usize> {
    octopuses: [[u8; N]; N],
}

type Pos = (usize, usize);

impl<const N: usize> std::str::FromStr for Grid<N> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_line<const N: usize>(line: &str) -> Result<[u8; N], &'static str> {
            line.trim()
                .chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or("Bad digit")?
                        .try_into()
                        .map_err(|_| "Digit too large")
                })
                .collect::<Result<Vec<u8>, _>>()?
                .try_into()
                .map_err(|_| "Wrong line size")
        }
        let octopuses: [[u8; N]; N] = s
            .trim()
            .lines()
            .map(parse_line)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "Wrong number of lines")?;
        Ok(Grid { octopuses })
    }
}

impl<const N: usize> From<[[u8; N]; N]> for Grid<N> {
    fn from(octopuses: [[u8; N]; N]) -> Self {
        Grid { octopuses }
    }
}

impl<const N: usize> std::fmt::Debug for Grid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::with_capacity(11 * 12);
        str += "Grid:";
        for row in self.octopuses.iter() {
            str.push('\n');
            str.push('\t');
            for n in row {
                str.push((48 + *n) as char);
            }
        }
        f.write_str(&str)
    }
}

impl<const N: usize> Grid<N> {
    pub fn evolve(&mut self) {
        let mut flashed = Vec::default();
        let mut to_visit = VecDeque::default();

        /// Checks if the octopus in the given position flashes. If so, it increases the energy of any
        /// neighbor that has not flashed yet and updates the queue to check them.
        fn propagate_flash<const N: usize>(
            this: &mut Grid<N>,
            pos: Pos,
            to_visit: &mut VecDeque<Pos>,
            flashed: &mut Vec<Pos>,
        ) {
            if flashed.contains(&pos) {
                return;
            }
            if this.octopuses[pos.0][pos.1] > 9 {
                flashed.push(pos);
                let (i, j) = pos;
                for delta_i in 0..=2 {
                    for delta_j in 0..=2 {
                        let ni = match (i + 1).checked_sub(delta_i) {
                            Some(ni) => ni,
                            None => continue,
                        };
                        let nj = match (j + 1).checked_sub(delta_j) {
                            Some(nj) => nj,
                            None => continue,
                        };
                        if ni != i || nj != j {
                            if let Some(Some(n)) =
                                this.octopuses.get_mut(ni).map(|row| row.get_mut(nj))
                            {
                                let pos = (ni, nj);
                                if !flashed.contains(&pos) {
                                    *n += 1;
                                }
                                if !to_visit.contains(&pos) {
                                    to_visit.push_back(pos)
                                }
                            }
                        }
                    }
                }
            }
        }

        // Find the first flashing octopuses
        for i in 0..N {
            for j in 0..N {
                // First the energy of each octopus increases by one
                self.octopuses[i][j] += 1;
                if self.octopuses[i][j] > 9 {
                    to_visit.push_back((i, j));
                }
            }
        }

        // Now find any recursive flashing octopus
        while let Some(pos) = to_visit.pop_front() {
            propagate_flash(self, pos, &mut to_visit, &mut flashed)
        }

        // Send any flashing octopus to 0
        for (i, j) in flashed {
            assert!(self.octopuses[i][j] > 9);
            self.octopuses[i][j] = 0;
        }
    }

    fn count_flashes(&self) -> usize {
        self.octopuses
            .iter()
            .flat_map(|row| row.iter().filter(|&&n| n == 0))
            .count()
    }

    fn evolve_counting_flashes(&mut self, count: &mut usize) {
        self.evolve();
        *count += self.count_flashes()
    }

    fn evolve_counting_flashes_n_times(&mut self, count: &mut usize, times: usize) {
        for _ in 0..times {
            self.evolve_counting_flashes(count)
        }
    }

    fn is_synchronized(&self) -> bool {
        self.octopuses.iter().all(|row| row.iter().all(|&n| n == 0))
    }

    fn evolve_until_synchronized(&mut self) -> usize {
        let mut steps = 0;
        loop {
            self.evolve();
            steps += 1;
            if self.is_synchronized() {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        ";

        let expected = [
            [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];

        assert_eq!(
            input.parse::<Grid::<10>>().expect("Input is ok"),
            expected.into()
        );
    }

    #[test]
    fn small_evolve() {
        let mut input = "
                11111
                19991
                19191
                19991
                11111"
            .parse::<Grid<5>>()
            .expect("input is ok");

        let expected = "
                34543
                40004
                50005
                40004
                34543"
            .parse::<Grid<5>>()
            .expect("input is ok");

        input.evolve();
        assert_eq!(input, expected);
    }

    #[test]
    fn test_evolve() {
        let mut input = "
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "
        .parse::<Grid<10>>()
        .expect("input is ok");

        let steps = [
            "6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637",
            "8807476555
            5089087054
            8597889608
            8485769600
            8700908800
            6600088989
            6800005943
            0000007456
            9000000876
            8700006848",
            "0050900866
            8500800575
            9900000039
            9700000041
            9935080063
            7712300000
            7911250009
            2211130000
            0421125000
            0021119000",
            "2263031977
            0923031697
            0032221150
            0041111163
            0076191174
            0053411122
            0042361120
            5532241122
            1532247211
            1132230211",
            "4484144000
            2044144000
            2253333493
            1152333274
            1187303285
            1164633233
            1153472231
            6643352233
            2643358322
            2243341322",
            "5595255111
            3155255222
            3364444605
            2263444496
            2298414396
            2275744344
            2264583342
            7754463344
            3754469433
            3354452433",
            "6707366222
            4377366333
            4475555827
            3496655709
            3500625609
            3509955566
            3486694453
            8865585555
            4865580644
            4465574644",
            "7818477333
            5488477444
            5697666949
            4608766830
            4734946730
            4740097688
            6900007564
            0000009666
            8000004755
            6800007755",
            "9060000644
            7800000976
            6900000080
            5840000082
            5858000093
            6962400000
            8021250009
            2221130009
            9111128097
            7911119976",
            "0481112976
            0031112009
            0041112504
            0081111406
            0099111306
            0093511233
            0442361130
            5532252350
            0532250600
            0032240000",
        ]
        .map(str::parse::<Grid<10>>);

        for grid_result in steps {
            let grid = grid_result.expect("Step is ok");
            input.evolve();
            assert_eq!(input, grid);
        }
    }

    #[test]
    fn test_count_flashes() {
        let mut input = "
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "
        .parse::<Grid<10>>()
        .expect("input is ok");

        let mut count = 0;
        input.evolve_counting_flashes_n_times(&mut count, 10);
        assert_eq!(count, 204);

        input.evolve_counting_flashes_n_times(&mut count, 100 - 10);
        assert_eq!(count, 1656);
    }

    #[test]
    fn test_synchronizaed() {
        let mut input = "
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "
        .parse::<Grid<10>>()
        .expect("input is ok");

        assert_eq!(input.evolve_until_synchronized(), 195);
    }
}
