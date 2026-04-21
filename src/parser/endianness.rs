/// Byte order for SAS numeric and integer fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    /// Little-endian byte order.
    Little,
    /// Big-endian byte order.
    Big,
}

impl Endianness {
    /// Decode an endianness marker from the parser's normalized byte value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Little),
            1 => Some(Self::Big),
            _ => None,
        }
    }

    /// Return the parser's normalized byte marker for this endianness.
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Little => 0,
            Self::Big => 1,
        }
    }

    /// Report whether the byte order is little-endian.
    pub fn is_little(&self) -> bool {
        matches!(self, Self::Little)
    }

    /// Report whether the byte order is big-endian.
    pub fn is_big(&self) -> bool {
        matches!(self, Self::Big)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        assert_eq!(Endianness::from_u8(0), Some(Endianness::Little));
        assert_eq!(Endianness::from_u8(1), Some(Endianness::Big));
        assert_eq!(Endianness::from_u8(2), None);
    }

    #[test]
    fn test_as_u8() {
        assert_eq!(Endianness::Little.as_u8(), 0);
        assert_eq!(Endianness::Big.as_u8(), 1);
    }

    #[test]
    fn test_is_little() {
        assert!(Endianness::Little.is_little());
        assert!(!Endianness::Big.is_little());
    }

    #[test]
    fn test_is_big() {
        assert!(Endianness::Big.is_big());
        assert!(!Endianness::Little.is_big());
    }
}
