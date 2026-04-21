use super::NumericValue;

/// Decoded logical cell value.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedValue {
    /// Numeric cell without a stronger semantic type.
    Numeric(NumericValue),
    /// Character cell decoded to UTF-8.
    Character(String),
    /// SAS date cell.
    Date(NumericValue),
    /// SAS time cell.
    Time(NumericValue),
    /// SAS datetime cell.
    DateTime(NumericValue),
    /// SAS duration cell.
    Duration(NumericValue),
}

impl ParsedValue {
    /// Return the numeric payload for any numeric-compatible variant.
    pub fn numeric(&self) -> Option<&NumericValue> {
        match self {
            Self::Numeric(value)
            | Self::Date(value)
            | Self::Time(value)
            | Self::DateTime(value)
            | Self::Duration(value) => Some(value),
            Self::Character(_) => None,
        }
    }

    /// Return the character payload when this is a character value.
    pub fn character(&self) -> Option<&str> {
        match self {
            Self::Character(value) => Some(value.as_str()),
            Self::Numeric(_)
            | Self::Date(_)
            | Self::Time(_)
            | Self::DateTime(_)
            | Self::Duration(_) => None,
        }
    }
}
