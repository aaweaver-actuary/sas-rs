/// Compression signature advertised by SAS metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasCompressionSignature {
    /// No compression signature.
    None,
    /// Row-level RLE compression signature.
    Rle,
    /// Zlib-based compression signature.
    Zlib,
}

impl SasCompressionSignature {
    /// Return the raw signature string for this compression marker.
    pub fn as_code(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Rle => "SASYZCRL",
            Self::Zlib => "SASYZCR2",
        }
    }

    /// Decode a raw SAS compression signature string.
    pub fn from_code(code: &str) -> Self {
        match code {
            "SASYZCRL" => Self::Rle,
            "SASYZCR2" => Self::Zlib,
            _ => Self::None,
        }
    }

    /// Report whether no compression signature is present.
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Report whether the signature indicates any compression.
    pub fn is_compressed(&self) -> bool {
        !self.is_none()
    }

    /// Report whether the signature indicates RLE compression.
    pub fn is_rle(&self) -> bool {
        matches!(self, Self::Rle)
    }

    /// Report whether the signature indicates zlib compression.
    pub fn is_zlib(&self) -> bool {
        matches!(self, Self::Zlib)
    }

    /// Return all recognized raw compression signature strings.
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
