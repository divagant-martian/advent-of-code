use super::{Point, Scanner};

const VIEW_RANGE: isize = 1000;

impl Scanner {
    pub fn sees(&self, point: &Point) -> bool {
        let Point { x, y, z } = point;
        let Point {
            x: x0,
            y: y0,
            z: z0,
        } = self.origin;
        (x - x0).abs() <= VIEW_RANGE && (y - y0).abs() <= VIEW_RANGE && (z - z0).abs() <= VIEW_RANGE
    }

    pub fn big_matches(&self, other: &Self) -> Option<Self> {
        for transform in super::transform::TRANSFORMATIONS.into_iter() {
            if let Some(new_other) = self.matches(other, transform) {
                return Some(new_other);
            }
        }
        None
    }

    pub fn transform(&self, transform: impl Fn(Point) -> Point + Copy) -> Self {
        let beacons = self
            .beacons
            .clone()
            .into_iter()
            .map(transform)
            .collect::<_>();
        Scanner {
            origin: transform(self.origin),
            beacons,
        }
    }

    pub fn translate(&self, vec: &Point) -> Self {
        self.transform(|p| p + vec)
    }

    pub fn matches(&self, other: &Self, transform: fn(Point) -> Point) -> Option<Self> {
        let mut matching_points = 0;
        let transformed = other.transform(transform);
        for a in &self.beacons {
            for b in &transformed.beacons {
                // assume a = b
                let diff = *a - b;
                let transformed = transformed.translate(&diff);
                let matches_from_self = transformed
                    .beacons
                    .iter()
                    .all(|p| !self.sees(&p) || self.beacons.contains(&p));
                if !matches_from_self {
                    // Try next matching
                    continue;
                }

                let matches_from_other = self
                    .beacons
                    .iter()
                    .all(|p| !transformed.sees(&p) || transformed.beacons.contains(&p));
                if !matches_from_other {
                    continue;
                }

                // We got here so they seem to match. Count how many
                let match_count = transformed.beacons.iter().filter(|p| self.sees(p)).count();
                if match_count >= 12 {
                    let repr = transformed
                        .beacons
                        .iter()
                        .filter(|p| self.sees(&(diff + p)))
                        .map(|Point { x, y, z }| format!("{},{},{}", x, y, z))
                        .collect::<Vec<_>>()
                        .join("\n");
                    println!("A matches with B with {} points:\n{}", match_count, repr);
                    println!("0 moved to {:?}", diff);
                    return Some(transformed);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_matches() {
        let input = "
            --- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401

            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390
        ";
        let scanners = input
            .trim()
            .split("\n\n")
            .map(Scanner::from_str)
            .collect::<Result<Vec<Scanner>, _>>()
            .unwrap();

        assert!(scanners[0].big_matches(&scanners[1]).is_some());
    }
}
