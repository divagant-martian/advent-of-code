use crate::Points;

impl std::fmt::Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::with_capacity((self.max_y * self.max_x) as usize);
        const VIEW_RANGE: isize = 3;
        for y in self.min_y - VIEW_RANGE..=self.max_y + VIEW_RANGE {
            for x in self.min_x - VIEW_RANGE..=self.max_x + VIEW_RANGE {
                if self.points.contains(&(y, x)) || (self.horizon_lit && self.in_horizon((y, x))) {
                    repr.push('#');
                } else {
                    repr.push('.');
                }
            }
            repr.push('\n');
        }
        f.write_str(&repr)?;
        f.write_fmt(format_args!(
            "min_x:{}, max_x:{}, min_y:{} max_y:{}",
            self.min_x, self.max_x, self.min_y, self.max_y
        ))
    }
}
