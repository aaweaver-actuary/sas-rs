#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasMissingTag {
    Dot,
    Underscore,
    Letter(char),
}

impl SasMissingTag {
    pub fn code(&self) -> char {
        match self {
            Self::Dot => '.',
            Self::Underscore => '_',
            Self::Letter(tag) => *tag,
        }
    }

    pub fn from_code(tag: char) -> Option<Self> {
        match tag {
            '.' => Some(Self::Dot),
            '_' => Some(Self::Underscore),
            'A'..='Z' => Some(Self::Letter(tag)),
            _ => None,
        }
    }
}
