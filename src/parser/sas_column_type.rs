use super::constants::{SAS7BDAT_COLUMN_TYPE_CHR, SAS7BDAT_COLUMN_TYPE_NUM};

/// Physical SAS column storage kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SasColumnType {
    /// Numeric storage backed by SAS floating-point cells.
    Numeric,
    /// Character storage backed by encoded text bytes.
    Character,
}

impl SasColumnType {
    /// Return the raw SAS column-type code for this variant.
    pub fn as_code(&self) -> u8 {
        match self {
            SasColumnType::Numeric => SAS7BDAT_COLUMN_TYPE_NUM,
            SasColumnType::Character => SAS7BDAT_COLUMN_TYPE_CHR,
        }
    }

    /// Decode a raw SAS column-type code.
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            SAS7BDAT_COLUMN_TYPE_NUM => Some(SasColumnType::Numeric),
            SAS7BDAT_COLUMN_TYPE_CHR => Some(SasColumnType::Character),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_as_code() {
        assert_eq!(SasColumnType::Numeric.as_code(), SAS7BDAT_COLUMN_TYPE_NUM);
        assert_eq!(SasColumnType::Character.as_code(), SAS7BDAT_COLUMN_TYPE_CHR);
    }

    #[test]
    fn test_from_code() {
        assert_eq!(
            SasColumnType::from_code(SAS7BDAT_COLUMN_TYPE_NUM),
            Some(SasColumnType::Numeric)
        );
        assert_eq!(
            SasColumnType::from_code(SAS7BDAT_COLUMN_TYPE_CHR),
            Some(SasColumnType::Character)
        );
    }

    #[test]
    fn test_from_code_invalid() {
        assert_eq!(SasColumnType::from_code(0x03), None);
    }

    #[test]
    fn test_column_type_codes_match_column_type() {
        assert_eq!(SasColumnType::Numeric.as_code(), SAS7BDAT_COLUMN_TYPE_NUM);
        assert_eq!(SasColumnType::Character.as_code(), SAS7BDAT_COLUMN_TYPE_CHR);
    }

    #[test]
    fn test_column_type_codes_method() {
        assert_eq!(SasColumnType::Numeric.as_code(), SAS7BDAT_COLUMN_TYPE_NUM);
        assert_eq!(SasColumnType::Character.as_code(), SAS7BDAT_COLUMN_TYPE_CHR);
    }
}
