#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Cucumber {
    Right,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Herd<const X: usize, const Y: usize> {
    cucumbers: [[Option<Cucumber>; X]; Y],
}

impl Cucumber {
    pub fn from_char(c: char) -> Result<Option<Cucumber>, &'static str> {
        match c {
            '>' => Ok(Some(Cucumber::Right)),
            'v' => Ok(Some(Cucumber::Down)),
            '.' => Ok(None),
            _ => Err("Unknown char"),
        }
    }
}

impl<const X: usize, const Y: usize> Herd<X, Y> {
    pub fn parse(input: &str) -> Result<Self, &'static str> {
        let mut cucumbers = [[None; X]; Y];
        for (y, line) in input.trim().lines().enumerate() {
            for (x, c) in line.trim().char_indices() {
                cucumbers[y][x] = Cucumber::from_char(c)?;
            }
        }
        Ok(Herd { cucumbers })
    }

    pub fn move_right(&mut self) -> bool {
        let original = self.clone();
        let mut changed = false;
        for y in 0..Y {
            for x in 0..X {
                if let Some(Cucumber::Right) = original[(x, y)] {
                    if original[(x + 1, y)].is_none() {
                        self[(x + 1, y)] = self[(x, y)].take();
                        changed = true;
                    }
                }
            }
        }
        changed
    }

    pub fn move_down(&mut self) -> bool {
        let original = self.clone();
        let mut changed = false;
        for y in 0..Y {
            for x in 0..X {
                if let Some(Cucumber::Down) = original[(x, y)] {
                    if original[(x, y + 1)].is_none() {
                        self[(x, y + 1)] = self[(x, y)].take();
                        changed = true;
                    }
                }
            }
        }
        changed
    }

    pub fn evolve(&mut self) -> usize {
        let mut count = 0;
        while {
            let right = self.move_right();
            let down = self.move_down();
            count += 1;
            right || down
        } {
            println!("Step: {}\n{}", count, self);
        }
        count
    }
}

impl<const X: usize, const Y: usize> std::ops::Index<(usize, usize)> for Herd<X, Y> {
    type Output = Option<Cucumber>;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cucumbers[y % Y][x % X]
    }
}

impl<const X: usize, const Y: usize> std::ops::IndexMut<(usize, usize)> for Herd<X, Y> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cucumbers[y % Y][x % X]
    }
}

impl<const X: usize, const Y: usize> std::fmt::Display for Herd<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = String::with_capacity((X + 1) * Y);
        for y in 0..Y {
            for x in 0..X {
                let c = match self[(x, y)] {
                    Some(Cucumber::Down) => 'v',
                    Some(Cucumber::Right) => '>',
                    None => '.',
                };
                repr.push(c);
            }
            repr.push('\n');
        }
        repr.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
            >.
            .v
        ";
        assert_eq!(
            Herd::<2, 2>::parse(input),
            Ok(Herd {
                cucumbers: [[Some(Cucumber::Right), None], [None, Some(Cucumber::Down)]]
            })
        );
    }
}
