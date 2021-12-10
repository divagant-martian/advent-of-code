#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    Open(Left),
    Close(Right),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Left {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Right {
    A,
    B,
    C,
    D,
}

impl std::convert::TryFrom<char> for Syntax {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match Left::try_from(value).map(Syntax::Open) {
            Err(value) => Right::try_from(value).map(Syntax::Close),
            ok => ok,
        }
    }
}

impl std::convert::TryFrom<char> for Left {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Left::A),
            '[' => Ok(Left::B),
            '{' => Ok(Left::C),
            '<' => Ok(Left::D),
            wat => Err(wat),
        }
    }
}

impl std::convert::TryFrom<char> for Right {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ')' => Ok(Right::A),
            ']' => Ok(Right::B),
            '}' => Ok(Right::C),
            '>' => Ok(Right::D),
            wat => Err(wat),
        }
    }
}

impl Left {
    pub fn char(&self) -> char {
        match self {
            Left::A => '(',
            Left::B => '[',
            Left::C => '{',
            Left::D => '<',
        }
    }

    pub fn close(&self) -> Right {
        match self {
            Left::A => Right::A,
            Left::B => Right::B,
            Left::C => Right::C,
            Left::D => Right::D,
        }
    }
}

impl Right {
    pub fn char(&self) -> char {
        match self {
            Right::A => ')',
            Right::B => ']',
            Right::C => '}',
            Right::D => '>',
        }
    }
}

impl std::fmt::Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.char()))
    }
}

impl std::fmt::Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.char()))
    }
}

impl std::fmt::Debug for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl std::fmt::Debug for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
