use std::collections::HashSet;

use super::{Fold, Paper};

impl std::str::FromStr for Paper {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dots = s
            .trim()
            .lines()
            .map(|l| {
                l.trim().split_once(',').map(|(x_str, y_str)| {
                    let x = x_str.parse::<usize>()?;
                    let y = y_str.parse::<usize>()?;
                    Ok((x, y))
                })
            })
            .collect::<Option<Result<HashSet<_>, std::num::ParseIntError>>>()
            .ok_or("Format is wrong")?
            .map_err(|_| "Nums could not be parsed")?;

        Ok(Paper { dots })
    }
}

impl std::str::FromStr for Fold {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = s
            .trim()
            .strip_prefix("fold along ")
            .ok_or("Wrong format: missing fold instruction prefix")?;

        if let Some(x_str) = instruction.strip_prefix("x=") {
            let x = x_str
                .parse::<usize>()
                .map_err(|_| "Failed to parse num in x=num")?;
            Ok(Fold::V(x))
        } else if let Some(y_str) = instruction.strip_prefix("y=") {
            let y = y_str
                .parse::<usize>()
                .map_err(|_| "Failed to parse num in y=num")?;
            Ok(Fold::H(y))
        } else {
            Err("missing direction (x|y)=")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_paper() {
        let paper = "
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
        ";
        let paper = paper.parse::<Paper>().unwrap();
        assert_eq!(paper.dots.len(), 18);
        assert_eq!(
            paper.dots,
            vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_parse_fold() {
        let fold = "fold along y=7".parse::<Fold>().unwrap();
        assert_eq!(fold, Fold::H(7));
        let fold = "fold along x=5".parse::<Fold>().unwrap();
        assert_eq!(fold, Fold::V(5));
    }
}
