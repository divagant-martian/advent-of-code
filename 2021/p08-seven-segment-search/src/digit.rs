use Digit::*;

use crate::signal::Signal::{self, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Digit {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}

impl Digit {
    pub const ALL: &'static [Self] = &[D0, D1, D2, D3, D4, D5, D6, D7, D8, D9];

    pub const fn signals(&self) -> &[Signal] {
        match self {
            D0 => &[A, B, C, E, F, G],
            D1 => &[C, F],
            D2 => &[A, C, D, E, G],
            D3 => &[A, C, D, F, G],
            D4 => &[B, C, D, F],
            D5 => &[A, B, D, F, G],
            D6 => &[A, B, D, E, F, G],
            D7 => &[A, C, F],
            D8 => Signal::ALL,
            D9 => &[A, B, C, D, F, G],
        }
    }
}
