use Signal::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl std::convert::From<char> for Signal {
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

pub fn print_signals(signals: &[Signal]) {
    let mut digit = String::with_capacity(12);
    digit.push(' ');

    for s in [A, B, D, C, E, G, F] {
        if signals.contains(&s) {
            digit.push(s.as_char());
        } else {
            digit.push(' ');
        }

        if s.enter() {
            digit.push('\n');
        }
    }

    print!("{}", digit);
}
