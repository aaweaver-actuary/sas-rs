#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SasPageType {
    Meta,
    Data,
    Mix,
    Amd,
    Mask,
    Compressed,
}

impl SasPageType {
    pub fn from_code(value: u16) -> Option<Self> {
        match value {
            0x0001 => Some(Self::Meta),
            0x0002 => Some(Self::Data),
            0x0003 => Some(Self::Mix),
            0x0004 => Some(Self::Amd),
            0x0005 => Some(Self::Mask),
            0x0006 => Some(Self::Compressed),
            _ => None,
        }
    }

    pub fn as_code(&self) -> u16 {
        match self {
            Self::Meta => 0x0001,
            Self::Data => 0x0002,
            Self::Mix => 0x0003,
            Self::Amd => 0x0004,
            Self::Mask => 0x0005,
            Self::Compressed => 0x0006,
        }
    }

    pub fn is_meta(&self) -> bool {
        matches!(self, Self::Meta)
    }

    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data)
    }

    pub fn is_mix(&self) -> bool {
        matches!(self, Self::Mix)
    }

    pub fn is_amd(&self) -> bool {
        matches!(self, Self::Amd)
    }

    pub fn is_mask(&self) -> bool {
        matches!(self, Self::Mask)
    }

    pub fn is_compressed(&self) -> bool {
        matches!(self, Self::Compressed)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_page_type_from_code() {
        assert_eq!(SasPageType::from_code(0x0001), Some(SasPageType::Meta));
        assert_eq!(SasPageType::from_code(0x0002), Some(SasPageType::Data));
        assert_eq!(SasPageType::from_code(0x0003), Some(SasPageType::Mix));
        assert_eq!(SasPageType::from_code(0x0004), Some(SasPageType::Amd));
        assert_eq!(SasPageType::from_code(0x0005), Some(SasPageType::Mask));
        assert_eq!(
            SasPageType::from_code(0x0006),
            Some(SasPageType::Compressed)
        );
        assert_eq!(SasPageType::from_code(0x0007), None);
    }

    #[test]
    fn test_page_type_as_code() {
        assert_eq!(SasPageType::Meta.as_code(), 0x0001);
        assert_eq!(SasPageType::Data.as_code(), 0x0002);
        assert_eq!(SasPageType::Mix.as_code(), 0x0003);
        assert_eq!(SasPageType::Amd.as_code(), 0x0004);
        assert_eq!(SasPageType::Mask.as_code(), 0x0005);
        assert_eq!(SasPageType::Compressed.as_code(), 0x0006);
    }

    #[test]
    fn test_page_type_checks() {
        let meta = SasPageType::Meta;
        let data = SasPageType::Data;
        let mix = SasPageType::Mix;
        let amd = SasPageType::Amd;
        let mask = SasPageType::Mask;
        let compressed = SasPageType::Compressed;

        assert!(meta.is_meta());
        assert!(!meta.is_data());
        assert!(!meta.is_mix());
        assert!(!meta.is_amd());
        assert!(!meta.is_mask());
        assert!(!meta.is_compressed());

        assert!(data.is_data());
        assert!(!data.is_meta());
        assert!(!data.is_mix());
        assert!(!data.is_amd());
        assert!(!data.is_mask());
        assert!(!data.is_compressed());

        assert!(mix.is_mix());
        assert!(!mix.is_data());
        assert!(!mix.is_amd());
        assert!(!mix.is_mask());
        assert!(!mix.is_compressed());
        assert!(!mix.is_meta());

        assert!(amd.is_amd());
        assert!(!amd.is_data());
        assert!(!amd.is_mix());
        assert!(!amd.is_meta());
        assert!(!amd.is_mask());
        assert!(!amd.is_compressed());

        assert!(mask.is_mask());
        assert!(!mask.is_data());
        assert!(!mask.is_mix());
        assert!(!mask.is_amd());
        assert!(!mask.is_meta());
        assert!(!mask.is_compressed());

        assert!(compressed.is_compressed());
        assert!(!compressed.is_data());
        assert!(!compressed.is_mix());
        assert!(!compressed.is_amd());
        assert!(!compressed.is_meta());
        assert!(!compressed.is_mask());
    }
}
