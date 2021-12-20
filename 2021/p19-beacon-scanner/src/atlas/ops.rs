use super::{Point, Scanner};

impl std::ops::Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::AddAssign for Scanner {
    fn add_assign(&mut self, rhs: Self) {
        self.beacons = self.beacons.union(&rhs.beacons).cloned().collect();
    }
}
