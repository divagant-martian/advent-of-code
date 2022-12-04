use crate::{round::Outcome, shape::Shape};

pub struct Strategy {
    steps: Vec<(Shape, Shape)>,
}

impl std::str::FromStr for Strategy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps: Result<Vec<(Shape, Shape)>, Self::Err> = s
            .lines()
            .map(|l| {
                let mut chars = l.chars();
                let other: Shape = chars.next().unwrap().try_into()?;
                chars.next();
                let me = match chars.next().unwrap() {
                    'X' => {
                        // we need to lose
                        other.wins_against()
                    }
                    'Y' => {
                        // draw
                        other
                    }
                    'Z' => {
                        // we need to win
                        other.lose_against()
                    }
                    _ => panic!("wrong strategy"),
                };
                // let me: Shape = chars.next().unwrap().try_into()?;
                Ok((other, me))
            })
            .collect();
        Ok(Strategy { steps: steps? })
    }
}

impl Strategy {
    pub fn execute(&self) -> impl Iterator<Item = Outcome> + '_ {
        self.steps
            .iter()
            .map(|(other, me)| Outcome::new(*other, *me))
    }
}
