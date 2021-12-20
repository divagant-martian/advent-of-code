use super::{Num, NumTree, Role};

impl std::ops::Add for NumTree {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut inner = self.inner;
        inner.extend(rhs.inner.into_iter());
        for num in inner.iter_mut() {
            num.depth += 1;
        }
        NumTree { inner }
    }
}

impl std::ops::AddAssign for NumTree {
    fn add_assign(&mut self, rhs: Self) {
        self.inner.extend(rhs.inner.into_iter());
        for num in self.inner.iter_mut() {
            num.depth += 1;
        }
    }
}

impl NumTree {
    fn sliding_condition(&self, slice_len: usize, condition: fn(&[Num]) -> bool) -> Option<usize> {
        let mut pos = 0;
        for slice in self.inner.windows(slice_len) {
            if condition(slice) {
                return Some(pos);
            }
            pos += 1;
        }
        None
    }

    pub fn explode_once(&mut self) -> bool {
        if let Some(pos) = self.sliding_condition(2, |pair| {
            let left = pair[0];
            let right = pair[1];
            left.depth > 4
                && left.depth == right.depth
                && left.role == Role::Left
                && right.role == Role::Right
        }) {
            let right = self.inner.remove(pos + 1);
            let left = self.inner[pos];
            let new_depth = left.depth - 1;
            let mut is_right = false;

            if let Some(left_left) = pos.checked_sub(1).and_then(|pos| self.inner.get_mut(pos)) {
                left_left.n += left.n;
                if left_left.role == Role::Left && left_left.depth == new_depth {
                    is_right = true;
                }
            }
            if let Some(right_right) = self.inner.get_mut(pos + 1) {
                right_right.n += right.n;
            }

            match is_right {
                false => {
                    self.inner[pos] = Num {
                        n: 0,
                        depth: new_depth,
                        role: Role::Left,
                    }
                }
                true => {
                    self.inner[pos] = Num {
                        n: 0,
                        depth: new_depth,
                        role: Role::Right,
                    }
                }
            }

            true
        } else {
            false
        }
    }

    pub fn split_once(&mut self) -> bool {
        if let Some(pos) = self.sliding_condition(1, |s| s[0].n >= 10) {
            let Num { n, role: _, depth } = self.inner[pos];
            self.inner[pos] = Num {
                n: n / 2,
                role: Role::Left,
                depth: depth + 1,
            };
            self.inner.insert(
                pos + 1,
                Num {
                    n: n - n / 2,
                    role: Role::Right,
                    depth: depth + 1,
                },
            );

            true
        } else {
            false
        }
    }

    pub fn reduce(&mut self) {
        while self.explode_once() || self.split_once() {}
    }
}
