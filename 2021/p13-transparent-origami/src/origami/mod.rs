use std::collections::HashSet;

mod parse;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Fold {
    /// Horizontal fold
    H(usize),
    /// Vertical fold
    V(usize),
}

pub struct Paper {
    dots: HashSet<(usize, usize)>,
}

impl Fold {
    pub fn after_fold_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        let (x, y) = pos;
        match self {
            Fold::H(axis) => {
                // fold the paper up (for horizontal y=... lines)
                if y >= *axis {
                    (x, 2 * axis - y)
                } else {
                    (x, y)
                }
            }
            Fold::V(axis) => {
                // fold the paper left (for vertical x=... lines)
                if x >= *axis {
                    (2 * axis - x, y)
                } else {
                    (x, y)
                }
            }
        }
    }
}

impl Paper {
    pub fn fold(&mut self, fold: Fold) {
        self.dots = self
            .dots
            .drain()
            .map(|pos| fold.after_fold_pos(pos))
            .collect();
    }

    /// Get a reference to the paper's dots.
    pub fn dots(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.dots.iter().cloned()
    }
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ((min_x, max_x), (min_y, max_y)) = self.dots.iter().fold(
            ((0, 0), (0, 0)),
            |((min_x, max_x), (min_y, max_y)), &(x, y)| {
                ((x.min(min_x), x.max(max_x)), (y.min(min_y), y.max(max_y)))
            },
        );
        let mut repr = String::with_capacity((max_x - min_x) * (1 + max_y - min_y));
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let x_right = x.checked_add(1);
                let x_left = x.checked_sub(1);
                let y_up = y.checked_sub(1);
                let y_down = y.checked_add(1);
                if self.dots.contains(&(x, y)) {
                    // Full point
                    repr.push('█');
                    continue;
                }
                let mut pushed = false;
                for (maybe_x, maybe_y, char) in [
                    (x_right, y_up, '◥'),
                    (x_right, y_down, '◢'),
                    (x_left, y_up, '◤'),
                    (x_left, y_down, '◣'),
                ] {
                    if let Some(xn) = maybe_x {
                        if let Some(yn) = maybe_y {
                            if self.dots.contains(&(x, yn)) && self.dots.contains(&(xn, y)) {
                                repr.push(char);
                                pushed = true;
                                break;
                            }
                        }
                    }
                }

                if !pushed {
                    repr.push(' ');
                }
            }
            repr.push('\n');
        }
        f.write_str(&repr)
    }
}

pub fn parse_input(input: &str) -> Result<(Paper, Vec<Fold>), &'static str> {
    let (dots_input, fold_input) = input
        .trim()
        .split_once("\n\n")
        .ok_or("Input should have two parts")?;

    let paper = dots_input.parse::<Paper>()?;
    let folds = fold_input
        .lines()
        .map(str::parse::<Fold>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((paper, folds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold() {
        let fold = Fold::H(7);
        assert_eq!(
            fold.after_fold_pos((3, 5)),
            (3, 5),
            "horizontal fold should not affect point in the upper plane"
        );
        assert_eq!(
            fold.after_fold_pos((3, 7)),
            (3, 7),
            "horizontal fold should not affect point in the fold line"
        );
        assert_eq!(
            fold.after_fold_pos((3, 9)),
            (3, 5),
            "horizontal fold should move point to upper plane"
        );

        let fold = Fold::V(5);
        assert_eq!(
            fold.after_fold_pos((3, 5)),
            (3, 5),
            "vertical fold should not affect point in the left plane"
        );
        assert_eq!(
            fold.after_fold_pos((5, 7)),
            (5, 7),
            "vertical fold should not affect point in the fold line"
        );
        assert_eq!(
            fold.after_fold_pos((9, 9)),
            (1, 9),
            "vertical fold should move point to left plane"
        );
    }

    #[test]
    fn test_fold_paper() {
        let (mut paper, folds) = parse_input(
            "
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5
        ",
        )
        .unwrap();
        for f in folds {
            paper.fold(f);
            assert_eq!(paper.dots.len(), 17, "Count for fold {:?}", f);
            break;
        }
    }
}
