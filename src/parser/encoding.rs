use crate::parser::constants::{
    SAS7BDAT_DEFAULT_ENCODING_CODE, SAS7BDAT_LATIN1_ENCODING_CODE, SAS7BDAT_UTF8_ENCODING_CODE,
    SAS7BDAT_WINDOWS_1252_ENCODING_CODE,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sas7bdatEncoding {
    Default,
    Utf8,
    Latin1,
    Windows1252,
}

impl Sas7bdatEncoding {
    pub fn as_code(&self) -> u8 {
        match self {
            Self::Default => SAS7BDAT_DEFAULT_ENCODING_CODE,
            Self::Utf8 => SAS7BDAT_UTF8_ENCODING_CODE,
            Self::Latin1 => SAS7BDAT_LATIN1_ENCODING_CODE,
            Self::Windows1252 => SAS7BDAT_WINDOWS_1252_ENCODING_CODE,
        }
    }

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

    pub fn is_default(&self) -> bool {
        matches!(self, Self::Default)
    }

    pub fn is_utf8(&self) -> bool {
        matches!(self, Self::Utf8)
    }

    pub fn is_latin1(&self) -> bool {
        matches!(self, Self::Latin1)
    }

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
