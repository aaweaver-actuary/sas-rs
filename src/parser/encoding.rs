use crate::parser::constants::{
    SAS7BDAT_DEFAULT_ENCODING_CODE, SAS7BDAT_LATIN1_ENCODING_CODE, SAS7BDAT_UTF8_ENCODING_CODE,
    SAS7BDAT_WINDOWS_1252_ENCODING_CODE,
};

/// SAS text encoding codes recognized by the parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sas7bdatEncoding {
    /// Default Windows-1252-compatible encoding marker.
    Default,
    /// UTF-8 encoding marker.
    Utf8,
    /// Latin-1 encoding marker.
    Latin1,
    /// Windows-1252 encoding marker.
    Windows1252,
}

impl Sas7bdatEncoding {
    /// Return the raw SAS encoding code for this variant.
    pub fn as_code(&self) -> u8 {
        match self {
            Self::Default => SAS7BDAT_DEFAULT_ENCODING_CODE,
            Self::Utf8 => SAS7BDAT_UTF8_ENCODING_CODE,
            Self::Latin1 => SAS7BDAT_LATIN1_ENCODING_CODE,
            Self::Windows1252 => SAS7BDAT_WINDOWS_1252_ENCODING_CODE,
        }
    }

    /// Decode a raw SAS encoding code.
    pub fn from_code(code: u8) -> Option<Self> {
        if code == SAS7BDAT_DEFAULT_ENCODING_CODE {
            Some(Self::Default)
        } else if code == SAS7BDAT_UTF8_ENCODING_CODE {
            Some(Self::Utf8)
        } else if code == SAS7BDAT_LATIN1_ENCODING_CODE {
            Some(Self::Latin1)
        } else if code == SAS7BDAT_WINDOWS_1252_ENCODING_CODE {
            Some(Self::Windows1252)
        } else {
            None
        }
    }

    /// Report whether this encoding is the default marker.
    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    /// Report whether this encoding is UTF-8.
    pub fn is_utf8(&self) -> bool {
        matches!(self, Self::Utf8)
    }

    /// Report whether this encoding is Latin-1.
    pub fn is_latin1(&self) -> bool {
        matches!(self, Self::Latin1)
    }

    /// Report whether this encoding is Windows-1252.
    pub fn is_windows_1252(&self) -> bool {
        matches!(self, Self::Windows1252)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_default() {
        let encoding = Sas7bdatEncoding::Default;
        assert!(encoding.is_default());
        assert!(!encoding.is_utf8());
        assert!(!encoding.is_latin1());
        assert!(!encoding.is_windows_1252());
    }

    #[test]
    fn test_is_utf8() {
        let encoding = Sas7bdatEncoding::Utf8;
        assert!(encoding.is_utf8());
        assert!(!encoding.is_default());
        assert!(!encoding.is_latin1());
        assert!(!encoding.is_windows_1252());
    }

    #[test]
    fn test_is_latin1() {
        let encoding = Sas7bdatEncoding::Latin1;
        assert!(encoding.is_latin1());
        assert!(!encoding.is_default());
        assert!(!encoding.is_utf8());
        assert!(!encoding.is_windows_1252());
    }

    #[test]
    fn test_is_windows_1252() {
        let encoding = Sas7bdatEncoding::Windows1252;
        assert!(encoding.is_windows_1252());
        assert!(!encoding.is_default());
        assert!(!encoding.is_utf8());
        assert!(!encoding.is_latin1());
    }
}
