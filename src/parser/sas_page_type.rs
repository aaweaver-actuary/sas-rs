use super::constants::{
    SAS7BDAT_AMD_PAGE_TYPE_CODE, SAS7BDAT_DATA_PAGE_TYPE_CODE, SAS7BDAT_MASK_PAGE_TYPE_CODE,
    SAS7BDAT_META_PAGE_TYPE_CODE, SAS7BDAT_MIX_PAGE_TYPE_CODE,
};

/// Classified SAS page type derived from the raw page header bits.
/// Classified SAS page type derived from the raw page header bits.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SasPageType {
    /// Metadata page.
    /// Metadata page.
    Meta,
    /// Data-only page.
    /// Data-only page.
    Data,
    /// Mixed metadata and data page.
    /// Mixed metadata and data page.
    Mix,
    /// AMD page.
    /// AMD page.
    Amd,
    /// Unrecognized page type bits.
    /// Unrecognized page type bits.
    Unknown(u16),
}

impl SasPageType {
    /// Decode a page type from the raw page-type field.
    pub fn from_code(value: u16) -> Self {
        Self::from_page_type(value)
    }

    /// Decode a page type after masking out auxiliary page flags.
    /// Decode a page type after masking out auxiliary page flags.
    pub fn from_page_type(value: u16) -> Self {
        match value & SAS7BDAT_MASK_PAGE_TYPE_CODE {
            SAS7BDAT_META_PAGE_TYPE_CODE => Self::Meta,
            SAS7BDAT_DATA_PAGE_TYPE_CODE => Self::Data,
            SAS7BDAT_MIX_PAGE_TYPE_CODE => Self::Mix,
            SAS7BDAT_AMD_PAGE_TYPE_CODE => Self::Amd,
            other => Self::Unknown(other),
        }
    }

    /// Return the canonical code for this logical page type.
    /// Return the canonical code for this logical page type.
    pub fn as_code(&self) -> u16 {
        match self {
            Self::Meta => SAS7BDAT_META_PAGE_TYPE_CODE,
            Self::Data => SAS7BDAT_DATA_PAGE_TYPE_CODE,
            Self::Mix => SAS7BDAT_MIX_PAGE_TYPE_CODE,
            Self::Amd => SAS7BDAT_AMD_PAGE_TYPE_CODE,
            Self::Unknown(code) => *code,
        }
    }

    /// Report whether this page is a metadata page.
    /// Report whether this page is a metadata page.
    pub fn is_meta(&self) -> bool {
        matches!(self, Self::Meta)
    }

    /// Report whether this page is a data page.
    /// Report whether this page is a data page.
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data)
    }

    /// Report whether this page is a mixed metadata/data page.
    /// Report whether this page is a mixed metadata/data page.
    pub fn is_mix(&self) -> bool {
        matches!(self, Self::Mix)
    }

    /// Report whether this page is an AMD page.
    /// Report whether this page is an AMD page.
    pub fn is_amd(&self) -> bool {
        matches!(self, Self::Amd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_type_from_code_uses_masked_sas_codes() {
        assert_eq!(SasPageType::from_code(0x0000), SasPageType::Meta);
        assert_eq!(SasPageType::from_code(0x0100), SasPageType::Data);
        assert_eq!(SasPageType::from_code(0x0200), SasPageType::Mix);
        assert_eq!(SasPageType::from_code(0x0400), SasPageType::Amd);
        assert_eq!(SasPageType::from_code(0x0A00), SasPageType::Unknown(0x0A00));
    }
}
