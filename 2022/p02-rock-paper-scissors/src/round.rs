use crate::shape::Shape;

pub enum Outcome {
    /// Roun was a draw.
    Draw(Shape),
    /// Round was lost using this shape.
    Lost(Shape),
    /// Round was won using this shape
    Won(Shape),
}

impl Outcome {
    pub fn new(other: Shape, me: Shape) -> Outcome {
        match (other, me) {
            (Shape::Paper, me @ Shape::Rock)
            | (Shape::Rock, me @ Shape::Scissors)
            | (Shape::Scissors, me @ Shape::Paper) => Outcome::Lost(me),
            (Shape::Rock, me @ Shape::Paper)
            | (Shape::Paper, me @ Shape::Scissors)
            | (Shape::Scissors, me @ Shape::Rock) => Outcome::Won(me),
            (Shape::Rock, me @ Shape::Rock)
            | (Shape::Paper, me @ Shape::Paper)
            | (Shape::Scissors, me @ Shape::Scissors) => Outcome::Draw(me),
        }
    }

    pub fn score(&self) -> usize {
        match self {
            Outcome::Draw(shape) => 3 + shape.score(),
            Outcome::Lost(shape) => 0 + shape.score(),
            Outcome::Won(shape) => 6 + shape.score(),
        }
    }
}
