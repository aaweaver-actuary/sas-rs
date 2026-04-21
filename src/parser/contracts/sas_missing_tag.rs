/// SAS special-missing value tag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasMissingTag {
    /// Standard `.` missing value.
    Dot,
    /// Underscore `_` missing value.
    Underscore,
    /// Letter-tagged missing value such as `.A`.
    Letter(char),
}

impl SasMissingTag {
    /// Return the tag code stored for this missing value.
    pub fn code(&self) -> char {
        match self {
            Self::Dot => '.',
            Self::Underscore => '_',
            Self::Letter(tag) => *tag,
        }
    }

    /// Decode a SAS missing-value tag from its code.
    pub fn from_code(tag: char) -> Option<Self> {
        match tag {
            '.' => Some(Self::Dot),
            '_' => Some(Self::Underscore),
            'A'..='Z' => Some(Self::Letter(tag)),
            _ => None,
        }
    }
}
