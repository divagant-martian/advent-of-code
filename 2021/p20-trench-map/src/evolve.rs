use std::collections::HashSet;

use crate::{Code, Points};

pub fn neighbors((y0, x0): &(isize, isize)) -> impl '_ + Iterator<Item = (isize, isize)> {
    (y0 - 1..=y0 + 1).flat_map(move |y| (x0 - 1..=x0 + 1).map(move |x| (y, x)))
}

impl Points {
    pub fn in_horizon(&self, n: (isize, isize)) -> bool {
        let (y, x) = n;
        y >= self.max_y + 2 || y <= self.min_y - 2 || x >= self.max_x + 2 || x <= self.min_x - 2
    }

    pub fn decode_around(&self, center: (isize, isize), code: &Code) -> bool {
        let mut preimage = 0;
        for n in neighbors(&center) {
            preimage *= 2;
            if self.points.contains(&n) || (self.horizon_lit && self.in_horizon(n)) {
                preimage += 1;
            }
        }
        println!("{:?}, {:09b}", center, preimage);
        code[preimage]
    }

    pub fn evolve(&mut self, code: &Code) {
        let mut new_points = HashSet::with_capacity(self.points.len());
        let mut min_x = self.min_x;
        let mut min_y = self.min_y;
        let mut max_x = self.max_x;
        let mut max_y = self.max_y;
        let view_range = 1;
        for y in self.min_y - view_range..=self.max_y + view_range {
            for x in self.min_x - view_range..=self.max_x + view_range {
                let p = (y, x);
                if self.decode_around(p, code) {
                    new_points.insert(p);
                    max_y = max_y.max(y);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    min_x = min_x.min(x);
                }
            }
        }

        self.points = new_points;
        self.max_y = max_y;
        self.max_x = max_x;
        self.min_x = min_x;
        self.min_y = min_y;
        self.horizon_lit = if self.horizon_lit { code[511] } else { code[0] };
    }
}
