use Signal::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Signal {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
}

impl From<char> for Signal {
    fn from(c: char) -> Self {
        match c {
            'a' => A,
            'b' => B,
            'c' => C,
            'd' => D,
            'e' => E,
            'f' => F,
            'g' => G,
            _ => unreachable!("bad char"),
        }
    }
}

impl Signal {
    pub const ALL: &'static [Self] = &[A, B, C, D, E, F, G];
    pub fn as_char(&self) -> char {
        match self {
            B | E | C | F => '|',
            A | D | G => '_',
        }
    }

    pub fn enter(&self) -> bool {
        match self {
            A | C | F => true,
            _ => false,
        }
    }
}
