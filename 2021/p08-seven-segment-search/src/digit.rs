use Digit::*;

use crate::signal::Signal::{self, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Digit {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
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

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
