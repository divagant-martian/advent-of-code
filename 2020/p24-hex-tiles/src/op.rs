#[allow(unused)]
/// Eisenstein integers.
#[derive(Debug)]
pub enum Op {
    NE,
    E,
    SE,
    NW,
    W,
    SW,
}

pub const OPS: [Op; 6] = [Op::NE, Op::E, Op::SE, Op::NW, Op::W, Op::SW];

impl Op {
    const fn shift(&self) -> (isize, isize) {
        match self {
            Op::NE => (1, 1),
            Op::E => (1, 0),
            Op::SE => (0, -1),
            Op::NW => (0, 1),
            Op::W => (-1, 0),
            Op::SW => (-1, -1),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct EisensteinInt {
    i: isize,
    omega: isize,
}

impl EisensteinInt {
    pub fn new(i: isize, omega: isize) -> Self {
        EisensteinInt { i, omega }
    }
}

impl std::ops::Add<&Op> for &EisensteinInt {
    type Output = EisensteinInt;

    fn add(self, op: &Op) -> EisensteinInt {
        let (shift_i, shift_omega) = op.shift();
        EisensteinInt {
            i: self.i + shift_i,
            omega: self.omega + shift_omega,
        }
    }
}

impl std::ops::AddAssign<Op> for EisensteinInt {
    fn add_assign(&mut self, op: Op) {
        let (shift_i, shift_omega) = op.shift();
        self.i += shift_i;
        self.omega += shift_omega;
    }
}
