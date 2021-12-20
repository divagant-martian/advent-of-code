use super::{Num, NumTree, Role};

impl std::str::FromStr for NumTree {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = Vec::new();
        let mut depth = 0;
        let mut role = Role::Left;
        for c in s.trim().chars() {
            match c {
                '[' => {
                    depth += 1;
                    role = Role::Left
                }
                ',' => role = Role::Right,
                ']' => depth -= 1,
                n => {
                    let n = n.to_digit(10).ok_or("Failed to parse digit")? as u8;
                    nums.push(Num { n, role, depth });
                }
            }
        }
        Ok(NumTree { inner: nums })
    }
}
