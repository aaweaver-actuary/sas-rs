#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

impl Endianness {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Little),
            1 => Some(Self::Big),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Little => 0,
            Self::Big => 1,
        }
    }

    pub fn is_little(&self) -> bool {
        matches!(self, Self::Little)
    }

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
