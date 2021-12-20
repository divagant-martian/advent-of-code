use std::str::FromStr;

use super::{Error, Scanner, Point};

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(',').map(isize::from_str);
        Ok(Point {
            x: parts
                .next()
                .ok_or("Missing x")?
                .map_err(|_| "Could not parse x")?,
            y: parts
                .next()
                .ok_or("Missing y")?
                .map_err(|_| "Could not parse y")?,
            z: parts
                .next()
                .ok_or("Missing z")?
                .map_err(|_| "Could not parse z")?,
        })
    }
}

impl FromStr for Scanner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        assert!(lines.next().unwrap().starts_with("---"));
        let beacons = lines.map(Point::from_str).collect::<Result<Vec<Point>, Error>>()?;
        Ok(Scanner { beacons })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        assert_eq!(Point::from_str("  5,6,-4  "), Ok(Point { x: 5, y: 6, z: -4 }));
    }

    #[test]
    fn test_parse_scanner() {
        let input = "
            --- scanner 3 ---
            -589,542,597
            605,-692,669
            -500,565,-823
            -660,373,557
            -458,-679,-417
            -488,449,543
            -626,468,-788
            338,-750,-386
            528,-832,-391
            562,-778,733
            -938,-730,414
            543,643,-506
            -524,371,-870
            407,773,750
            -104,29,83
            378,-903,-323
            -778,-728,485
            426,699,580
            -438,-605,-362
            -469,-447,-387
            509,732,623
            647,635,-688
            -868,-804,481
            614,-800,639
            595,780,-596
        ";
        let scanner = Scanner::from_str(input).unwrap();
        assert_eq!(scanner.beacons.len(), 25);
    }
}
