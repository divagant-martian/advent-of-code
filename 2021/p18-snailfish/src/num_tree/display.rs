use crate::num_tree::{Num, NumTree};

use super::Role;

impl std::fmt::Display for NumTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::with_capacity(self.inner.len());
        let mut current_depth = 0;

        for num in &self.inner {
            let Num { n, role, depth } = num;
            while current_depth > *depth {
                repr.push(']');
                current_depth -= 1;
            }
            match role {
                Role::Left => {
                    if current_depth == *depth {
                        repr.push(']');
                        current_depth -= 1;
                    }
                    if current_depth != 0 {
                        repr.push(',');
                    }
                    while current_depth < *depth {
                        repr.push('[');
                        current_depth += 1;
                    }
                    repr += &n.to_string();
                }
                Role::Right => {
                    repr.push(',');
                    repr += &n.to_string();
                    repr.push(']');
                    current_depth -= 1;
                }
            }
        }

        while current_depth > 0 {
            repr.push(']');
            current_depth -= 1;
        }
        f.write_str(&repr)
    }
}

impl std::fmt::Debug for NumTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter()).finish()
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}{}{}", self.n, self.role, self.depth))
    }
}

impl std::fmt::Debug for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Role::Left => "i",
            Role::Right => "d",
        })
    }
}
