#[derive(Debug, PartialEq, Eq)]
pub enum Cave<'a> {
    Start,
    Small(&'a str),
    Big(&'a str),
    End,
}

impl<'a> Cave<'a> {
    pub fn is_big(&self) -> bool {
        match self {
            Cave::Big(_) => true,
            Cave::Start | Cave::End | Cave::Small(_) => false,
        }
    }

    pub fn is_small(&self) -> bool {
        match self {
            Cave::Small(_) => true,
            Cave::Start | Cave::Big(_) | Cave::End => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Cave::Start => "start",
            Cave::Small(other) => other,
            Cave::Big(other) => other,
            Cave::End => "end",
        }
    }
}

impl<'a> std::fmt::Display for Cave<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a, 'b: 'a> std::convert::TryFrom<&'b str> for Cave<'a> {
    type Error = &'static str;

    fn try_from(s: &'b str) -> Result<Self, Self::Error> {
        match s.trim() {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            other => {
                if other.chars().all(|c| c.is_ascii_uppercase()) {
                    Ok(Cave::Big(other))
                } else if other.chars().all(|c| c.is_ascii_lowercase()) {
                    Ok(Cave::Small(other))
                } else {
                    Err("Mixed lowercase and upercase")
                }
            }
        }
    }
}
