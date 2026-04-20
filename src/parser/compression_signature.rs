#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasCompressionSignature {
    None,
    Rle,
    Zlib,
}

impl SasCompressionSignature {
    pub fn as_code(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Rle => "SASYZCRL",
            Self::Zlib => "SASYZCR2",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code {
            "SASYZCRL" => Self::Rle,
            "SASYZCR2" => Self::Zlib,
            _ => Self::None,
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_compressed(&self) -> bool {
        !self.is_none()
    }

    pub fn is_rle(&self) -> bool {
        matches!(self, Self::Rle)
    }

    pub fn is_zlib(&self) -> bool {
        matches!(self, Self::Zlib)
    }

    pub fn codes() -> &'static [&'static str] {
        &["", "SASYZCRL", "SASYZCR2"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_signature_codes() {
        let codes = SasCompressionSignature::codes();
        for &code in codes {
            let signature = SasCompressionSignature::from_code(code);
            assert_eq!(signature.as_code(), code);
        }
    }

    #[test]
    fn test_compression_signature_helpers() {
        let none = SasCompressionSignature::None;
        let rle = SasCompressionSignature::Rle;
        let zlib = SasCompressionSignature::Zlib;

        assert!(none.is_none());
        assert!(!none.is_compressed());
        assert!(!none.is_rle());
        assert!(!none.is_zlib());

        assert!(!rle.is_none());
        assert!(rle.is_compressed());
        assert!(rle.is_rle());
        assert!(!rle.is_zlib());

        assert!(!zlib.is_none());
        assert!(zlib.is_compressed());
        assert!(!zlib.is_rle());
        assert!(zlib.is_zlib());
    }
}
